use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServiceSettingsTemplateSecretNoSqlModelType {
    Static,
    Generated,
}

#[my_no_sql_macros::my_no_sql_entity("servicesettingstemplatesecret")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSettingsTemplateSecretNoSqlModel {
    pub template_id: String,
    pub secret_value: Option<String>,
    pub secret_name: String,
    pub secret_type: ServiceSettingsTemplateSecretNoSqlModelType,
}

impl ServiceSettingsTemplateSecretNoSqlModel {
    pub fn generate_partition_key(template_id: &str) -> String {
        template_id.to_string()
    }
}
