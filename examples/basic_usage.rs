use amqp_rust::{Frame, FRAME_HEARTBEAT, CLASS_BASIC, METHOD_BASIC_PUBLISH};
use bytes::{Bytes, BytesMut};

fn main() {
    println!("AMQP Protocol Parsing Library Example\n");

    // Example 1: Parse a heartbeat frame
    println!("=== Example 1: Heartbeat Frame ===");
    let mut buf = BytesMut::new();
    let heartbeat = Frame::Heartbeat;
    heartbeat.serialize(&mut buf).unwrap();
    
    println!("Serialized heartbeat frame: {:?}", buf);
    
    let parsed_frame = Frame::parse(&mut buf).unwrap().unwrap();
    println!("Parsed frame: {:?}", parsed_frame);
    assert!(parsed_frame.is_heartbeat());
    println!("✓ Successfully parsed heartbeat frame\n");

    // Example 2: Parse a body frame
    println!("=== Example 2: Body Frame ===");
    let message = b"Hello, AMQP!";
    let body_frame = Frame::Body {
        channel: 1,
        data: Bytes::from(&message[..]),
    };
    
    let mut buf = BytesMut::new();
    body_frame.serialize(&mut buf).unwrap();
    println!("Serialized body frame: {:?}", buf);
    
    let parsed_frame = Frame::parse(&mut buf).unwrap().unwrap();
    println!("Parsed frame: {:?}", parsed_frame);
    
    if let Frame::Body { channel, data } = parsed_frame {
        println!("Channel: {}", channel);
        println!("Message: {}", String::from_utf8_lossy(&data));
        assert_eq!(&data[..], message);
    }
    println!("✓ Successfully parsed body frame\n");

    // Example 3: Parse a method frame
    println!("=== Example 3: Method Frame ===");
    let method_frame = Frame::Method {
        channel: 1,
        class_id: CLASS_BASIC,
        method_id: METHOD_BASIC_PUBLISH,
        arguments: Bytes::new(),
    };
    
    let mut buf = BytesMut::new();
    method_frame.serialize(&mut buf).unwrap();
    println!("Serialized method frame: {:?}", buf);
    
    let parsed_frame = Frame::parse(&mut buf).unwrap().unwrap();
    println!("Parsed frame: {:?}", parsed_frame);
    assert!(parsed_frame.is_method());
    println!("✓ Successfully parsed method frame\n");

    println!("All examples completed successfully!");
}
