//! Single-instance lock acquisition, IPC routing, and stale-lock recovery.

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use ronin::{acquire_instance, plan_incoming_launch, IncomingLaunch, LaunchIntent};
use tempfile::TempDir;

fn runtime_dir() -> (TempDir, PathBuf) {
    let temp = TempDir::new().expect("temp");
    let dir = temp.path().join("runtime");
    std::fs::create_dir_all(&dir).unwrap();
    (temp, dir)
}

#[test]
fn first_launch_should_acquire_instance_lock_as_primary() {
    let (_temp, dir) = runtime_dir();
    let intent = LaunchIntent::OpenPersisted {
        attach_paths: Vec::new(),
    };

    let acquired = acquire_instance(&dir, &intent).expect("acquire");
    assert!(
        acquired.is_primary(),
        "first launch must become the primary instance"
    );
}

#[test]
fn second_launch_should_detect_existing_instance_and_hand_off() {
    let (_temp, dir) = runtime_dir();
    let intent_a = LaunchIntent::OpenPersisted {
        attach_paths: Vec::new(),
    };
    let mut primary = acquire_instance(&dir, &intent_a)
        .expect("primary")
        .into_primary()
        .expect("primary handle");

    let dir2 = dir.clone();
    let secondary = thread::spawn(move || {
        let intent_b = LaunchIntent::NewThread {
            attach_paths: Vec::new(),
        };
        acquire_instance(&dir2, &intent_b).expect("secondary")
    });

    // Primary must accept the IPC connection.
    let mut incoming = None;
    for _ in 0..40 {
        if let Some(msg) = primary.try_recv().expect("recv") {
            incoming = Some(msg);
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }

    let secondary_result = secondary.join().expect("join");
    assert!(
        secondary_result.is_handed_off(),
        "second launch must hand off to the primary and exit"
    );

    let incoming = incoming.expect("primary should receive routed intent");
    assert_eq!(
        incoming.intent,
        LaunchIntent::NewThread {
            attach_paths: Vec::new(),
        }
    );
    assert!(incoming.focus, "secondary launch should request focus");
}

#[test]
fn second_launch_should_route_attach_paths_and_quick_intent() {
    let (_temp, dir) = runtime_dir();
    let mut primary = acquire_instance(
        &dir,
        &LaunchIntent::OpenPersisted {
            attach_paths: Vec::new(),
        },
    )
    .unwrap()
    .into_primary()
    .unwrap();

    let dir2 = dir.clone();
    let secondary = thread::spawn(move || {
        acquire_instance(
            &dir2,
            &LaunchIntent::Quick {
                attach_paths: vec![PathBuf::from("/tmp/note.md")],
            },
        )
        .expect("hand off")
    });

    let mut incoming = None;
    for _ in 0..40 {
        if let Some(msg) = primary.try_recv().expect("recv") {
            incoming = Some(msg);
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
    assert!(secondary.join().unwrap().is_handed_off());

    let incoming = incoming.expect("routed");
    assert_eq!(
        incoming.intent,
        LaunchIntent::Quick {
            attach_paths: vec![PathBuf::from("/tmp/note.md")],
        }
    );
    let plan = plan_incoming_launch(&incoming);
    assert!(plan.open_quick_overlay);
    assert!(!plan.create_new_thread);
    assert!(plan.focus_window);
    assert_eq!(plan.attach_paths, vec![PathBuf::from("/tmp/note.md")]);
}

#[test]
fn stale_lock_should_allow_new_primary_after_crash() {
    let (_temp, dir) = runtime_dir();

    let primary = acquire_instance(
        &dir,
        &LaunchIntent::OpenPersisted {
            attach_paths: Vec::new(),
        },
    )
    .unwrap()
    .into_primary()
    .unwrap();

    // Simulate crash: drop primary without clean shutdown (Drop still runs in
    // Rust, so also leave a stale socket file as a crash would).
    let sock = primary.socket_path().to_path_buf();
    drop(primary);
    // Recreate a stale socket file path that would remain after a hard kill
    // where Drop might not run — bind a listener then forget it via leak of
    // only the path: write an empty placeholder file after removing.
    let _ = std::fs::remove_file(&sock);
    std::fs::write(&sock, b"").expect("stale sock placeholder");

    let recovered = acquire_instance(
        &dir,
        &LaunchIntent::NewThread {
            attach_paths: Vec::new(),
        },
    )
    .expect("recover after stale");
    assert!(
        recovered.is_primary(),
        "new instance must acquire lock after previous crash / stale socket"
    );
}

#[test]
fn plan_incoming_launch_should_map_intents_to_primary_actions() {
    let new = plan_incoming_launch(&IncomingLaunch {
        intent: LaunchIntent::NewThread {
            attach_paths: vec!["a.txt".into()],
        },
        focus: true,
    });
    assert!(new.create_new_thread);
    assert!(!new.prefer_ollama);
    assert!(new.focus_window);
    assert_eq!(new.attach_paths, vec![PathBuf::from("a.txt")]);

    let ollama = plan_incoming_launch(&IncomingLaunch {
        intent: LaunchIntent::OpenWithOllama {
            attach_paths: Vec::new(),
        },
        focus: true,
    });
    assert!(!ollama.create_new_thread);
    assert!(ollama.prefer_ollama);
    assert!(ollama.focus_window);
}
