use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("tenantsbrands")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantBrandConfig{
    pub id: String,
    pub tenant_id: String,
    pub about_page: Option<String>,
    pub policy_page: Option<String>,
    pub support_page: Option<String>,
    pub favicon: Option<String>,
    pub logo: Option<String>,
}

impl TenantBrandConfig {
    pub fn generate_partition_key() -> &'static str {
        "tb"
    }
}