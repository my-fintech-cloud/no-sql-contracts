use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("servicesettingstemplate")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSettingsTemplateNoSqlModel {
    pub id: String,
    pub service_id: String,
    pub template: String,
}

impl ServiceSettingsTemplateNoSqlModel {
    pub fn generate_partition_key() -> String {
        "st".to_string()
    }
}
