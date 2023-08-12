use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[readonly::make]
pub struct SQSAttributes {
    pub approximate_receive_count: String,
    pub approximate_first_receive_timestamp: DateTime<Utc>,
    pub message_deduplication_id: Option<String>,
    pub message_group_id: Option<String>,
    pub sender_id: String,
    pub sent_timestamp: DateTime<Utc>,
    pub sequence_number: Option<String>,
    pub aws_trace_header: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[readonly::make]
pub struct SqsMsgAttribute {
    pub string_value: Option<String>,
    pub binary_value: Option<String>,
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[readonly::make]
pub struct SQSRecord<T> {
    pub message_id: String,
    pub receipt_handle: String,
    pub body: T,
    pub attributes: SQSAttributes,
    pub message_attributes: HashMap<String, SqsMsgAttribute>,
    pub md5_of_body: String,
    pub event_source: String,
    pub event_source_arn: String,
    pub aws_region: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[readonly::make]
pub struct SQSEvent<T> {
    pub records: Vec<SQSRecord<T>>,
}
