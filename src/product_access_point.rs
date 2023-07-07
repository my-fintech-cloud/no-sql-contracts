use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProductAccessPointNoSqlType {
    Local,
    External,
}

#[my_no_sql_macros::my_no_sql_entity("productaccesspoints")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAccessPointNoSqlModel {
    pub id: String,
    pub product_id: String,
    pub access_point_type: ProductAccessPointNoSqlType,
    pub create_date: u64,
    pub create_process_id: String,
    pub update_date: u64,
    pub update_process_id: String,
    pub location: String,
    pub domain: String,
    pub cloud_provider: String,
}

impl ProductAccessPointNoSqlModel {
    pub fn generate_partition_key(tenant_id: &String) -> &str {
        &tenant_id
    }
}
