use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub results: Vec<NDCProduct>,
}

#[derive(Debug, Deserialize)]
pub struct NDCProduct {
    pub product_ndc: String,
    pub generic_name: String,
    pub labeler_name: String,
    pub brand_name: Option<String>,
    pub finished: bool,
    pub listing_expiration_date: Option<String>,
    pub marketing_category: String,
    pub dosage_form: String,
    pub spl_id: String,
    pub product_type: String,
    pub marketing_start_date: String,
    pub product_id: String,
    pub application_number: Option<String>,
    pub brand_name_base: Option<String>,
}
