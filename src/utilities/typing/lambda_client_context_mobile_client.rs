use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[readonly::make]
pub struct LambdaClientContextMobileClient {
    pub installation_id: String,
    pub app_title: String,
    pub app_version_name: String,
    pub app_version_code: String,
    pub app_package_name: String,
}
