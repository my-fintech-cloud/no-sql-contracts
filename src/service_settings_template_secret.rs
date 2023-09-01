use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServiceSettingsTemplateSecretNoSqlModelType {
    Static(String),
    Generated,
    InputByClient(ServiceSettingsTemplateSecretTypeDataType),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServiceSettingsTemplateSecretTypeDataType {
    String,
    Int,
    Float,
    File,
    Bool,
}

#[my_no_sql_macros::my_no_sql_entity("servicesettingstemplatesecret")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSettingsTemplateSecretNoSqlModel {
    pub secret_name: String,
    pub secret_type: ServiceSettingsTemplateSecretNoSqlModelType,
}

impl ServiceSettingsTemplateSecretNoSqlModel {
    pub fn generate_partition_key() -> String {
        "ts".to_string()
    }
}
