use amqp_rust::{Connection, Frame};
use bytes::BytesMut;

fn main() {
    println!("AMQP Connection Management Example\n");

    // Create a new connection
    let mut conn = Connection::new();
    println!("Created connection with defaults:");
    println!("  - Channel Max: {}", conn.channel_max());
    println!("  - Frame Max: {} bytes", conn.frame_max());
    println!("  - Heartbeat: {} seconds\n", conn.heartbeat());

    // Configure connection parameters
    conn.set_channel_max(256);
    conn.set_frame_max(65536);
    conn.set_heartbeat(30);
    
    println!("Updated connection parameters:");
    println!("  - Channel Max: {}", conn.channel_max());
    println!("  - Frame Max: {} bytes", conn.frame_max());
    println!("  - Heartbeat: {} seconds\n", conn.heartbeat());

    // Protocol header
    let header = Connection::protocol_header();
    println!("Protocol header: {:?}", header);
    println!("  - Magic: {}", String::from_utf8_lossy(&header[0..4]));
    println!("  - Version: {}.{}.{}\n", header[5], header[6], header[7]);

    // Create various frames
    println!("=== Creating Frames ===");
    
    let heartbeat = Connection::create_heartbeat();
    println!("Created heartbeat frame: {:?}", heartbeat);
    
    let channel_open = Connection::create_channel_open(1);
    println!("Created channel open frame: {:?}", channel_open);
    
    let channel_close = Connection::create_channel_close(1);
    println!("Created channel close frame: {:?}\n", channel_close);

    // Serialize and parse frames
    println!("=== Serializing and Parsing ===");
    let mut buf = BytesMut::new();
    
    heartbeat.serialize(&mut buf).unwrap();
    println!("Serialized heartbeat: {} bytes", buf.len());
    
    let parsed = Frame::parse(&mut buf).unwrap().unwrap();
    println!("Parsed back: {:?}", parsed);
    assert!(parsed.is_heartbeat());
    println!("✓ Heartbeat frame roundtrip successful\n");

    // Channel operations
    println!("=== Channel Operations ===");
    let mut buf = BytesMut::new();
    
    channel_open.serialize(&mut buf).unwrap();
    println!("Serialized channel open: {} bytes", buf.len());
    
    let parsed = Frame::parse(&mut buf).unwrap().unwrap();
    if let Frame::Method { channel, class_id, method_id, .. } = parsed {
        println!("Parsed Method Frame:");
        println!("  - Channel: {}", channel);
        println!("  - Class ID: {}", class_id);
        println!("  - Method ID: {}", method_id);
        println!("✓ Channel open frame parsed successfully");
    }

    println!("\nAll connection examples completed successfully!");
}
