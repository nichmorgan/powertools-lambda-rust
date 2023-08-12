use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type RawEvent = HashMap<String, Value>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct PartialItemFailures {
    pub item_identifier: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct PartialItemFailureResponse {
    pub batch_item_failures: Vec<PartialItemFailures>,
}
