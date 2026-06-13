use ronin_core::{MessageRole, RoninPaths, RoninSession, AttachmentKind};
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
    let msg = session.create_message(&thread.id, MessageRole::User, "hello").unwrap();

    let artifact = session.create_artifact(&thread.id, &msg.id, "Test Artifact", "content").unwrap();
    assert_eq!(artifact.title, "Test Artifact");
    assert_eq!(artifact.content, "content");
    
    let artifacts = session.list_artifacts(&thread.id).unwrap();
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].id, artifact.id);

    session.delete_artifact(&artifact.id).unwrap();
    assert!(session.list_artifacts(&thread.id).unwrap().is_empty());
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
    let msg = session.create_message(&thread.id, MessageRole::User, "hello").unwrap();

    let attachment = session.create_attachment(
        &msg.id,
        AttachmentKind::File,
        "main.rs",
        "text/rust",
        None,
        Some("/path/to/main.rs")
    ).unwrap();

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
