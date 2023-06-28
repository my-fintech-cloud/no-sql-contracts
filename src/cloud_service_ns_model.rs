use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[my_no_sql_macros::my_no_sql_entity("cloudservice")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudServiceNoSqlEntity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub depends_on: Vec<String>,
    pub k8s_deployment: String,
    pub docker_deployment: String,
    pub labels: HashMap<String, String>
}

impl CloudServiceNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "cs"
    }
} 