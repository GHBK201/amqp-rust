/// AMQP Protocol Constants

// Protocol version
pub const PROTOCOL_VERSION_MAJOR: u8 = 0;
pub const PROTOCOL_VERSION_MINOR: u8 = 9;
pub const PROTOCOL_VERSION_REVISION: u8 = 1;

// Frame types
pub const FRAME_METHOD: u8 = 1;
pub const FRAME_HEADER: u8 = 2;
pub const FRAME_BODY: u8 = 3;
pub const FRAME_HEARTBEAT: u8 = 8;

// Frame constants
pub const FRAME_END: u8 = 0xCE;
pub const FRAME_MIN_SIZE: usize = 4096;

// AMQP Class IDs
pub const CLASS_CONNECTION: u16 = 10;
pub const CLASS_CHANNEL: u16 = 20;
pub const CLASS_EXCHANGE: u16 = 40;
pub const CLASS_QUEUE: u16 = 50;
pub const CLASS_BASIC: u16 = 60;
pub const CLASS_TX: u16 = 90;

// Connection Method IDs
pub const METHOD_CONNECTION_START: u16 = 10;
pub const METHOD_CONNECTION_START_OK: u16 = 11;
pub const METHOD_CONNECTION_TUNE: u16 = 30;
pub const METHOD_CONNECTION_TUNE_OK: u16 = 31;
pub const METHOD_CONNECTION_OPEN: u16 = 40;
pub const METHOD_CONNECTION_OPEN_OK: u16 = 41;
pub const METHOD_CONNECTION_CLOSE: u16 = 50;
pub const METHOD_CONNECTION_CLOSE_OK: u16 = 51;

// Channel Method IDs
pub const METHOD_CHANNEL_OPEN: u16 = 10;
pub const METHOD_CHANNEL_OPEN_OK: u16 = 11;
pub const METHOD_CHANNEL_CLOSE: u16 = 40;
pub const METHOD_CHANNEL_CLOSE_OK: u16 = 41;

// Queue Method IDs
pub const METHOD_QUEUE_DECLARE: u16 = 10;
pub const METHOD_QUEUE_DECLARE_OK: u16 = 11;
pub const METHOD_QUEUE_BIND: u16 = 20;
pub const METHOD_QUEUE_BIND_OK: u16 = 21;
pub const METHOD_QUEUE_DELETE: u16 = 40;
pub const METHOD_QUEUE_DELETE_OK: u16 = 41;

// Basic Method IDs
pub const METHOD_BASIC_PUBLISH: u16 = 40;
pub const METHOD_BASIC_DELIVER: u16 = 60;
pub const METHOD_BASIC_GET: u16 = 70;
pub const METHOD_BASIC_GET_OK: u16 = 71;
pub const METHOD_BASIC_GET_EMPTY: u16 = 72;
pub const METHOD_BASIC_ACK: u16 = 80;
pub const METHOD_BASIC_CONSUME: u16 = 20;
pub const METHOD_BASIC_CONSUME_OK: u16 = 21;
