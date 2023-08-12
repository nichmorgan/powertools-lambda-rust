use serde::{Deserialize, Serialize};

use super::{
    lambda_client_context::LambdaClientContext, lambda_cognito_identity::LambdaCognitoIdentity,
};

#[derive(Serialize, Deserialize, Debug)]
#[readonly::make]
pub struct LambdaContext {
    pub function_name: String,
    pub function_version: String,
    pub invoked_function_arn: String,
    pub memory_limit_in_mb: usize,
    pub aws_request_id: String,
    pub log_group_name: String,
    pub log_stream_name: String,
    pub identity: LambdaCognitoIdentity,
    pub client_context: LambdaClientContext,
    pub get_remaining_time_in_millis: usize,
}
