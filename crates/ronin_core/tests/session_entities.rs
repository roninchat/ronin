use ronin_core::{AttachmentKind, MessageRole, RoninPaths, RoninSession};
use tempfile::TempDir;

fn setup_session() -> (RoninSession, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    (session, temp)
}

#[test]
fn session_artifact_crud() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().unwrap();
    let msg = session
        .create_message(&thread.id, MessageRole::User, "hello")
        .unwrap();

    let artifact = session
        .create_artifact(&thread.id, &msg.id, "Test Artifact", "content")
        .unwrap();
    assert_eq!(artifact.title, "Test Artifact");
    assert_eq!(artifact.content, "content");

    let artifacts = session.list_artifacts(&thread.id).unwrap();
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].id, artifact.id);

    session.delete_artifact(&artifact.id).unwrap();
    assert!(session.list_artifacts(&thread.id).unwrap().is_empty());
}

#[test]
fn session_create_snippet_artifact_should_preserve_fence_language() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().unwrap();
    let msg = session
        .create_message(
            &thread.id,
            MessageRole::Assistant,
            "```python\nprint('hi')\n```",
        )
        .unwrap();

    let artifact = session
        .create_snippet_artifact(
            &thread.id,
            &msg.id,
            "print example",
            "print('hi')",
            "python",
        )
        .unwrap();

    assert!(artifact.is_snippet());
    assert_eq!(artifact.kind, "snippet");
    assert_eq!(artifact.language.as_deref(), Some("python"));
    assert_eq!(artifact.content, "print('hi')");
}

#[test]
fn session_list_all_artifacts_across_threads() {
    let (session, _temp) = setup_session();
    let thread1 = session.create_thread().unwrap();
    let msg1 = session
        .create_message(&thread1.id, MessageRole::User, "hello")
        .unwrap();
    let thread2 = session.create_thread().unwrap();
    let msg2 = session
        .create_message(&thread2.id, MessageRole::User, "world")
        .unwrap();

    let art1 = session
        .create_artifact(&thread1.id, &msg1.id, "A1", "c1")
        .unwrap();
    let art2 = session
        .create_artifact(&thread2.id, &msg2.id, "A2", "c2")
        .unwrap();

    let all = session.list_all_artifacts().unwrap();
    assert_eq!(all.len(), 2);
    // newest first
    assert_eq!(all[0].id, art2.id);
    assert_eq!(all[1].id, art1.id);
}

#[test]
fn session_should_rename_and_edit_artifact_through_update() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().unwrap();
    let msg = session
        .create_message(&thread.id, MessageRole::User, "hello")
        .unwrap();
    let artifact = session
        .create_artifact(&thread.id, &msg.id, "Original", "old body")
        .unwrap();

    session
        .update_artifact(&artifact.id, "Renamed", "new body")
        .expect("update artifact");

    let listed = session.list_artifacts(&thread.id).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].title, "Renamed");
    assert_eq!(listed[0].content, "new body");
    assert_eq!(listed[0].id, artifact.id);
}

#[test]
fn session_memory_crud() {
    let (session, _temp) = setup_session();

    let memory = session.create_memory("Preference", "Use Rust").unwrap();
    assert_eq!(memory.title, "Preference");
    assert_eq!(memory.content, "Use Rust");

    let memories = session.list_memories().unwrap();
    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0].id, memory.id);

    session.delete_memory(&memory.id).unwrap();
    assert!(session.list_memories().unwrap().is_empty());
}

#[test]
fn session_attachment_crud() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().unwrap();
    let msg = session
        .create_message(&thread.id, MessageRole::User, "hello")
        .unwrap();

    let attachment = session
        .create_attachment(
            &msg.id,
            AttachmentKind::File,
            "main.rs",
            "text/rust",
            None,
            Some("/path/to/main.rs"),
        )
        .unwrap();

    assert_eq!(attachment.name, "main.rs");
    assert_eq!(attachment.mime_type, "text/rust");
    assert_eq!(attachment.kind, AttachmentKind::File);
    assert_eq!(attachment.path.as_deref(), Some("/path/to/main.rs"));

    let attachments = session.list_attachments(&msg.id).unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].id, attachment.id);

    session.delete_attachment(&attachment.id).unwrap();
    assert!(session.list_attachments(&msg.id).unwrap().is_empty());
}

#[test]
fn session_should_persist_image_and_screenshot_attachment_metadata() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().unwrap();
    let msg = session
        .create_message(&thread.id, MessageRole::User, "see these")
        .unwrap();

    let image = session
        .create_attachment(
            &msg.id,
            AttachmentKind::Image,
            "photo.png",
            "image/png",
            None,
            Some("/tmp/photo.png"),
        )
        .unwrap();
    let shot = session
        .create_attachment(
            &msg.id,
            AttachmentKind::Screenshot,
            "shot.png",
            "image/png",
            None,
            Some("/tmp/shot.png"),
        )
        .unwrap();

    let listed = session.list_attachments(&msg.id).unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].kind, AttachmentKind::Image);
    assert_eq!(listed[0].mime_type, "image/png");
    assert_eq!(listed[0].path.as_deref(), Some("/tmp/photo.png"));
    assert_eq!(listed[0].id, image.id);
    assert_eq!(listed[1].kind, AttachmentKind::Screenshot);
    assert_eq!(listed[1].mime_type, "image/png");
    assert_eq!(listed[1].path.as_deref(), Some("/tmp/shot.png"));
    assert_eq!(listed[1].id, shot.id);
}
