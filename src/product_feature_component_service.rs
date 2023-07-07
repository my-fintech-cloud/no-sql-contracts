use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExposeIpRule {
    Path(String),
    ThirdLvlDomain(String),
}

#[my_no_sql_macros::my_no_sql_entity("productcomponentservices")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductComponentServiceNoSqlEntity {
    pub id: String,
    pub name: String,
    pub product_id: String,
    pub feature_id: String,
    pub feature_component_id: String,
    pub description: String,
    pub k8s_deployment: String,
    pub docker_deployment: String,
    pub expose_ip_rule: Option<ExposeIpRule>,
    pub labels: HashMap<String, String>,
}

impl ProductComponentServiceNoSqlEntity {
    pub fn generate_partition_key(component_id: String) -> String {
        component_id
    }
}
