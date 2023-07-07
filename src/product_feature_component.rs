use serde::{Serialize, Deserialize};

#[my_no_sql_macros::my_no_sql_entity("productfeaturecomponents")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductFeatureComponentNoSqlModel {
    pub id: String,
    pub name: String,
    pub product_id: String,
    pub feature_id: String,
    pub description: String,
    pub create_date: u64,
    pub update_date: u64,
    pub create_process_id: String,
    pub update_process_id: String,
}

impl ProductFeatureComponentNoSqlModel{
    pub fn generate_partition_key(feature_id: &str) -> String{
        feature_id.to_string()
    }
}
