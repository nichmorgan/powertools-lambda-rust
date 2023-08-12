use std::collections::HashMap;

use super::lambda_client_context_mobile_client::LambdaClientContextMobileClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[readonly::make]
pub struct LambdaClientContext {
    pub client: LambdaClientContextMobileClient,
    pub custom: HashMap<String, Value>,
}
