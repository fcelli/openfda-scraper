use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub meta: Meta,
    pub results: Vec<NDCProduct>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub results: MetaResults,
}

#[derive(Debug, Deserialize)]
pub struct MetaResults {
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct NDCProduct {
    pub product_ndc: String,
    pub generic_name: Option<String>,
    pub labeler_name: String,
    pub brand_name: Option<String>,
    pub finished: bool,
    pub packaging: Vec<NDCPackage>,
    pub listing_expiration_date: Option<String>,
    pub marketing_category: String,
    pub dosage_form: String,
    pub product_type: String,
    pub marketing_start_date: String,
    pub application_number: Option<String>,
    pub brand_name_base: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NDCPackage {
    pub package_ndc: String,
    pub description: String,
    pub marketing_start_date: String,
    pub sample: Option<bool>,
}
