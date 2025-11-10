//! AMQP Frame parsing and serialization

use crate::constants::*;
use crate::error::{AmqpError, Result};
use crate::types::BasicProperties;
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// AMQP Frame Header
#[derive(Debug, Clone, PartialEq)]
pub struct FrameHeader {
    pub frame_type: u8,
    pub channel: u16,
    pub size: u32,
}

/// AMQP Frame types
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Method {
        channel: u16,
        class_id: u16,
        method_id: u16,
        arguments: Bytes,
    },
    Header {
        channel: u16,
        class_id: u16,
        weight: u16,
        body_size: u64,
        properties: Box<BasicProperties>,
    },
    Body {
        channel: u16,
        data: Bytes,
    },
    Heartbeat,
}

impl Frame {
    /// Parse a frame from bytes
    pub fn parse(buf: &mut BytesMut) -> Result<Option<Frame>> {
        // Need at least 7 bytes for frame header (type + channel + size)
        if buf.len() < 7 {
            return Ok(None);
        }

        let frame_type = buf[0];
        let channel = u16::from_be_bytes([buf[1], buf[2]]);
        let size = u32::from_be_bytes([buf[3], buf[4], buf[5], buf[6]]) as usize;

        // Check if we have the complete frame (header + payload + frame-end)
        if buf.len() < 7 + size + 1 {
            return Ok(None);
        }

        // Advance past header
        buf.advance(7);

        let frame = match frame_type {
            FRAME_METHOD => {
                if size < 4 {
                    return Err(AmqpError::ParseError("Method frame too small".to_string()));
                }

                let class_id = buf.get_u16();
                let method_id = buf.get_u16();
                let arguments = buf.split_to(size - 4).freeze();

                Frame::Method {
                    channel,
                    class_id,
                    method_id,
                    arguments,
                }
            }
            FRAME_HEADER => {
                if size < 12 {
                    return Err(AmqpError::ParseError("Header frame too small".to_string()));
                }

                let class_id = buf.get_u16();
                let weight = buf.get_u16();
                let body_size = buf.get_u64();
                
                // Parse properties (simplified - full implementation would parse property flags)
                let properties = Box::new(BasicProperties::default());
                buf.advance(size - 12);

                Frame::Header {
                    channel,
                    class_id,
                    weight,
                    body_size,
                    properties,
                }
            }
            FRAME_BODY => {
                let data = buf.split_to(size).freeze();
                Frame::Body { channel, data }
            }
            FRAME_HEARTBEAT => {
                buf.advance(size);
                Frame::Heartbeat
            }
            _ => return Err(AmqpError::InvalidFrameType(frame_type)),
        };

        // Check frame end marker
        let frame_end = buf.get_u8();
        if frame_end != FRAME_END {
            return Err(AmqpError::InvalidFrameEnd(frame_end));
        }

        Ok(Some(frame))
    }

    /// Serialize frame to bytes
    pub fn serialize(&self, buf: &mut BytesMut) -> Result<()> {
        match self {
            Frame::Method {
                channel,
                class_id,
                method_id,
                arguments,
            } => {
                let size = 4 + arguments.len();
                buf.put_u8(FRAME_METHOD);
                buf.put_u16(*channel);
                buf.put_u32(size as u32);
                buf.put_u16(*class_id);
                buf.put_u16(*method_id);
                buf.put_slice(arguments);
                buf.put_u8(FRAME_END);
            }
            Frame::Header {
                channel,
                class_id,
                weight,
                body_size,
                properties: _,
            } => {
                let size = 12; // Simplified: class_id(2) + weight(2) + body_size(8)
                buf.put_u8(FRAME_HEADER);
                buf.put_u16(*channel);
                buf.put_u32(size);
                buf.put_u16(*class_id);
                buf.put_u16(*weight);
                buf.put_u64(*body_size);
                buf.put_u8(FRAME_END);
            }
            Frame::Body { channel, data } => {
                buf.put_u8(FRAME_BODY);
                buf.put_u16(*channel);
                buf.put_u32(data.len() as u32);
                buf.put_slice(data);
                buf.put_u8(FRAME_END);
            }
            Frame::Heartbeat => {
                buf.put_u8(FRAME_HEARTBEAT);
                buf.put_u16(0);
                buf.put_u32(0);
                buf.put_u8(FRAME_END);
            }
        }
        Ok(())
    }

    /// Get the channel number for this frame
    pub fn channel(&self) -> u16 {
        match self {
            Frame::Method { channel, .. } => *channel,
            Frame::Header { channel, .. } => *channel,
            Frame::Body { channel, .. } => *channel,
            Frame::Heartbeat => 0,
        }
    }

    /// Check if this is a method frame
    pub fn is_method(&self) -> bool {
        matches!(self, Frame::Method { .. })
    }

    /// Check if this is a header frame
    pub fn is_header(&self) -> bool {
        matches!(self, Frame::Header { .. })
    }

    /// Check if this is a body frame
    pub fn is_body(&self) -> bool {
        matches!(self, Frame::Body { .. })
    }

    /// Check if this is a heartbeat frame
    pub fn is_heartbeat(&self) -> bool {
        matches!(self, Frame::Heartbeat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heartbeat_frame() {
        let mut buf = BytesMut::new();
        buf.put_u8(FRAME_HEARTBEAT); // frame type
        buf.put_u16(0);              // channel
        buf.put_u32(0);              // size
        buf.put_u8(FRAME_END);       // frame end

        let frame = Frame::parse(&mut buf).unwrap();
        assert!(frame.is_some());
        assert!(matches!(frame.unwrap(), Frame::Heartbeat));
    }

    #[test]
    fn test_serialize_heartbeat_frame() {
        let frame = Frame::Heartbeat;
        let mut buf = BytesMut::new();
        frame.serialize(&mut buf).unwrap();

        assert_eq!(buf[0], FRAME_HEARTBEAT);
        assert_eq!(buf[7], FRAME_END);
        assert_eq!(buf.len(), 8);
    }

    #[test]
    fn test_parse_method_frame() {
        let mut buf = BytesMut::new();
        buf.put_u8(FRAME_METHOD);    // frame type
        buf.put_u16(1);              // channel
        buf.put_u32(4);              // size
        buf.put_u16(CLASS_CONNECTION); // class id
        buf.put_u16(METHOD_CONNECTION_START); // method id
        buf.put_u8(FRAME_END);       // frame end

        let frame = Frame::parse(&mut buf).unwrap();
        assert!(frame.is_some());
        
        if let Some(Frame::Method { channel, class_id, method_id, .. }) = frame {
            assert_eq!(channel, 1);
            assert_eq!(class_id, CLASS_CONNECTION);
            assert_eq!(method_id, METHOD_CONNECTION_START);
        } else {
            panic!("Expected method frame");
        }
    }

    #[test]
    fn test_parse_body_frame() {
        let test_data = b"Hello, AMQP!";
        let mut buf = BytesMut::new();
        buf.put_u8(FRAME_BODY);           // frame type
        buf.put_u16(1);                   // channel
        buf.put_u32(test_data.len() as u32); // size
        buf.put_slice(test_data);         // body data
        buf.put_u8(FRAME_END);            // frame end

        let frame = Frame::parse(&mut buf).unwrap();
        assert!(frame.is_some());
        
        if let Some(Frame::Body { channel, data }) = frame {
            assert_eq!(channel, 1);
            assert_eq!(&data[..], test_data);
        } else {
            panic!("Expected body frame");
        }
    }

    #[test]
    fn test_serialize_body_frame() {
        let test_data = Bytes::from(&b"Hello, AMQP!"[..]);
        let frame = Frame::Body {
            channel: 1,
            data: test_data.clone(),
        };

        let mut buf = BytesMut::new();
        frame.serialize(&mut buf).unwrap();

        // Verify frame structure
        assert_eq!(buf[0], FRAME_BODY);
        assert_eq!(u16::from_be_bytes([buf[1], buf[2]]), 1); // channel
        assert_eq!(u32::from_be_bytes([buf[3], buf[4], buf[5], buf[6]]), test_data.len() as u32);
    }

    #[test]
    fn test_insufficient_data() {
        let mut buf = BytesMut::new();
        buf.put_u8(FRAME_HEARTBEAT);
        buf.put_u16(0);
        // Not enough data for complete frame

        let result = Frame::parse(&mut buf);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_invalid_frame_type() {
        let mut buf = BytesMut::new();
        buf.put_u8(99);              // invalid frame type
        buf.put_u16(0);              // channel
        buf.put_u32(0);              // size
        buf.put_u8(FRAME_END);       // frame end

        let result = Frame::parse(&mut buf);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::InvalidFrameType(99)));
    }

    #[test]
    fn test_invalid_frame_end() {
        let mut buf = BytesMut::new();
        buf.put_u8(FRAME_HEARTBEAT); // frame type
        buf.put_u16(0);              // channel
        buf.put_u32(0);              // size
        buf.put_u8(0xFF);            // invalid frame end

        let result = Frame::parse(&mut buf);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AmqpError::InvalidFrameEnd(0xFF)));
    }

    #[test]
    fn test_frame_channel() {
        let frame = Frame::Method {
            channel: 5,
            class_id: CLASS_BASIC,
            method_id: METHOD_BASIC_PUBLISH,
            arguments: Bytes::new(),
        };
        assert_eq!(frame.channel(), 5);

        let frame = Frame::Heartbeat;
        assert_eq!(frame.channel(), 0);
    }

    #[test]
    fn test_frame_type_checks() {
        let method_frame = Frame::Method {
            channel: 1,
            class_id: CLASS_BASIC,
            method_id: METHOD_BASIC_PUBLISH,
            arguments: Bytes::new(),
        };
        assert!(method_frame.is_method());
        assert!(!method_frame.is_header());
        assert!(!method_frame.is_body());
        assert!(!method_frame.is_heartbeat());

        let heartbeat_frame = Frame::Heartbeat;
        assert!(!heartbeat_frame.is_method());
        assert!(heartbeat_frame.is_heartbeat());
    }
}
