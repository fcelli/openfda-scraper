use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub meta: Meta,
    pub results: Vec<Product>,
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
pub struct Product {
    pub product_ndc: String,
    pub generic_name: Option<String>,
    pub labeler_name: String,
    pub brand_name: Option<String>,
    pub active_ingredients: Option<Vec<ActiveIngredient>>,
    pub finished: bool,
    pub packaging: Vec<Package>,
    pub listing_expiration_date: Option<String>,
    pub marketing_category: String,
    pub dosage_form: String,
    pub product_type: String,
    pub marketing_start_date: String,
    pub application_number: Option<String>,
    pub brand_name_base: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub package_ndc: String,
    pub description: String,
    pub marketing_start_date: String,
    pub sample: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ActiveIngredient {
    pub name: String,
    pub strength: Option<String>,
}
