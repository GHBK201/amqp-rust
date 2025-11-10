/// AMQP basic types

use std::collections::HashMap;

/// Field value types for AMQP
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Boolean(bool),
    ShortShortInt(i8),
    ShortShortUInt(u8),
    ShortInt(i16),
    ShortUInt(u16),
    LongInt(i32),
    LongUInt(u32),
    LongLongInt(i64),
    LongLongUInt(u64),
    Float(f32),
    Double(f64),
    DecimalValue(u8, u32),
    ShortString(String),
    LongString(Vec<u8>),
    FieldArray(Vec<FieldValue>),
    Timestamp(u64),
    FieldTable(FieldTable),
    Void,
}

/// Field table type
pub type FieldTable = HashMap<String, FieldValue>;

/// AMQP Properties for Basic class
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BasicProperties {
    pub content_type: Option<String>,
    pub content_encoding: Option<String>,
    pub headers: Option<FieldTable>,
    pub delivery_mode: Option<u8>,
    pub priority: Option<u8>,
    pub correlation_id: Option<String>,
    pub reply_to: Option<String>,
    pub expiration: Option<String>,
    pub message_id: Option<String>,
    pub timestamp: Option<u64>,
    pub type_: Option<String>,
    pub user_id: Option<String>,
    pub app_id: Option<String>,
    pub cluster_id: Option<String>,
}
