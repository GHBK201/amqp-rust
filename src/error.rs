use std::fmt;

/// Error types for AMQP parsing
#[derive(Debug, Clone, PartialEq)]
pub enum AmqpError {
    /// Insufficient data to parse
    InsufficientData,
    /// Invalid frame type
    InvalidFrameType(u8),
    /// Invalid frame end marker
    InvalidFrameEnd(u8),
    /// Parse error with message
    ParseError(String),
    /// IO error
    IoError(String),
}

impl fmt::Display for AmqpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AmqpError::InsufficientData => write!(f, "Insufficient data to parse frame"),
            AmqpError::InvalidFrameType(t) => write!(f, "Invalid frame type: {}", t),
            AmqpError::InvalidFrameEnd(e) => write!(f, "Invalid frame end marker: {:#x}, expected 0xCE", e),
            AmqpError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AmqpError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for AmqpError {}

pub type Result<T> = std::result::Result<T, AmqpError>;
