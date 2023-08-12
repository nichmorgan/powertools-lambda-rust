use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[readonly::make]
pub struct LambdaCognitoIdentity {
    pub cognito_identity_id: String,
    pub cognito_identity_pool_id: String,
}
