use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("cloudproduct")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloudProductNoSqlModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub create_process_id: String,
    pub last_update_process_id: String,
    pub create_date: u64,
    pub update_date: u64
}

impl CloudProductNoSqlModel {
    pub fn generate_partition_key() -> &'static str {
        "cp"
    }
} 