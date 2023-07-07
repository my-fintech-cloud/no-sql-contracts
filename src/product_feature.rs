use serde::{Serialize, Deserialize};

#[my_no_sql_macros::my_no_sql_entity("productfeatures")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductFeatureNoSqlModel {
    pub id: String,
    pub name: String,
    pub product_id: String,
    pub description: String,
    pub is_default: bool,
    pub create_date: u64,
    pub update_date: u64,
    pub create_process_id: String,
    pub update_process_id: String,
}

impl ProductFeatureNoSqlModel {
    pub fn generate_partition_key() -> &'static str {
        "pf"
    }
}