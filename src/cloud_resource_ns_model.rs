use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[my_no_sql_macros::my_no_sql_entity("cloudresources")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudResourceNoSqlEntity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub services_ids: Vec<String>,
    pub labels: HashMap<String, String>
}

impl CloudResourceNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "cr"
    }
}