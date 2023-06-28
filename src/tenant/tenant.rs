use serde::{Serialize, Deserialize};

#[my_no_sql_macros::my_no_sql_entity("tenants")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantNoSqlEntity{
    pub id: String,
    pub name: String,
    pub location: String,
    pub cloud_provider: String,
    pub create_process_id: String,
    pub last_update_process_id: String,
    pub create_date: u64,
    pub last_update_date: u64,
}


impl TenantNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "t"
    }
}