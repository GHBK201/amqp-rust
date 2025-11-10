//! Connection and protocol handshake utilities

use crate::constants::*;
use crate::frame::Frame;
use bytes::Bytes;

/// AMQP Protocol Header
pub const PROTOCOL_HEADER: [u8; 8] = [b'A', b'M', b'Q', b'P', 0, 0, 9, 1];

/// Helper struct for managing AMQP connections
#[derive(Debug)]
pub struct Connection {
    channel_max: u16,
    frame_max: u32,
    heartbeat: u16,
}

impl Connection {
    /// Create a new connection with default parameters
    pub fn new() -> Self {
        Self {
            channel_max: 0,
            frame_max: 131072, // 128KB default
            heartbeat: 60,     // 60 seconds default
        }
    }

    /// Set the maximum number of channels
    pub fn set_channel_max(&mut self, channel_max: u16) {
        self.channel_max = channel_max;
    }

    /// Get the maximum number of channels
    pub fn channel_max(&self) -> u16 {
        self.channel_max
    }

    /// Set the maximum frame size
    pub fn set_frame_max(&mut self, frame_max: u32) {
        self.frame_max = frame_max;
    }

    /// Get the maximum frame size
    pub fn frame_max(&self) -> u32 {
        self.frame_max
    }

    /// Set the heartbeat interval in seconds
    pub fn set_heartbeat(&mut self, heartbeat: u16) {
        self.heartbeat = heartbeat;
    }

    /// Get the heartbeat interval in seconds
    pub fn heartbeat(&self) -> u16 {
        self.heartbeat
    }

    /// Create the protocol header bytes
    pub fn protocol_header() -> &'static [u8; 8] {
        &PROTOCOL_HEADER
    }

    /// Create a heartbeat frame
    pub fn create_heartbeat() -> Frame {
        Frame::Heartbeat
    }

    /// Create a channel open method frame
    pub fn create_channel_open(channel: u16) -> Frame {
        Frame::Method {
            channel,
            class_id: CLASS_CHANNEL,
            method_id: METHOD_CHANNEL_OPEN,
            arguments: Bytes::new(),
        }
    }

    /// Create a channel close method frame
    pub fn create_channel_close(channel: u16) -> Frame {
        Frame::Method {
            channel,
            class_id: CLASS_CHANNEL,
            method_id: METHOD_CHANNEL_CLOSE,
            arguments: Bytes::new(),
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_defaults() {
        let conn = Connection::new();
        assert_eq!(conn.channel_max(), 0);
        assert_eq!(conn.frame_max(), 131072);
        assert_eq!(conn.heartbeat(), 60);
    }

    #[test]
    fn test_connection_setters() {
        let mut conn = Connection::new();
        conn.set_channel_max(100);
        conn.set_frame_max(65536);
        conn.set_heartbeat(30);

        assert_eq!(conn.channel_max(), 100);
        assert_eq!(conn.frame_max(), 65536);
        assert_eq!(conn.heartbeat(), 30);
    }

    #[test]
    fn test_protocol_header() {
        let header = Connection::protocol_header();
        assert_eq!(header, &PROTOCOL_HEADER);
        assert_eq!(&header[0..4], b"AMQP");
        assert_eq!(header[4], 0);
        assert_eq!(header[5], 0);
        assert_eq!(header[6], 9);
        assert_eq!(header[7], 1);
    }

    #[test]
    fn test_create_heartbeat() {
        let frame = Connection::create_heartbeat();
        assert!(frame.is_heartbeat());
    }

    #[test]
    fn test_create_channel_open() {
        let frame = Connection::create_channel_open(1);
        assert!(frame.is_method());
        
        if let Frame::Method { channel, class_id, method_id, .. } = frame {
            assert_eq!(channel, 1);
            assert_eq!(class_id, CLASS_CHANNEL);
            assert_eq!(method_id, METHOD_CHANNEL_OPEN);
        }
    }

    #[test]
    fn test_create_channel_close() {
        let frame = Connection::create_channel_close(1);
        assert!(frame.is_method());
        
        if let Frame::Method { channel, class_id, method_id, .. } = frame {
            assert_eq!(channel, 1);
            assert_eq!(class_id, CLASS_CHANNEL);
            assert_eq!(method_id, METHOD_CHANNEL_CLOSE);
        }
    }
}
