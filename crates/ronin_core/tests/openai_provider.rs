use ronin_core::{ChatProvider, ChatRequest, ChatStreamEvent, OllamaHealth, OllamaProvider, OpenAiCompatibleProvider};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

#[test]
fn openai_provider_should_report_healthy_when_models_endpoint_reachable() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"data\":[{\"id\":\"gpt-4\"}]}";
        stream.write_all(response.as_bytes()).unwrap();
    });

    let provider = OpenAiCompatibleProvider::new(format!("http://127.0.0.1:{}", port));
    // Provide a dummy API key via env for the test
    std::env::set_var("OPENAI_API_KEY", "test-key");
    
    assert_eq!(provider.check_health(), OllamaHealth::Online);
}

#[test]
fn openai_provider_should_stream_chat_responses() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        
        // Read headers until empty line
        loop {
            let mut header = String::new();
            reader.read_line(&mut header).unwrap();
            if header == "\r\n" { break; }
        }
        
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\n\r\n\
        data: {\"choices\":[{\"delta\":{\"content\":\"hello\"}}]}\n\n\
        data: {\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n\n\
        data: [DONE]\n\n";
        stream.write_all(response.as_bytes()).unwrap();
    });

    let provider = OpenAiCompatibleProvider::new(format!("http://127.0.0.1:{}", port));
    std::env::set_var("OPENAI_API_KEY", "test-key");
    
    let request = ChatRequest {
        model: "gpt-4".into(),
        messages: vec![],
        system_prompt: None,
    };
    
    let stream = provider.stream_chat(&request).expect("stream chat");
    let chunks: Vec<_> = stream.collect();
    
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], ChatStreamEvent::Chunk("hello".into()));
    assert_eq!(chunks[1], ChatStreamEvent::Chunk(" world".into()));
}
