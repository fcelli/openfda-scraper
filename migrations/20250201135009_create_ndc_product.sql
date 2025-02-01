CREATE TABLE IF NOT EXISTS ndc_product (
    product_ndc TEXT PRIMARY KEY,
    generic_name TEXT,
    labeler_name TEXT,
    brand_name TEXT NULL,
    finished BOOLEAN,
    listing_expiration_date TEXT NULL,
    marketing_category TEXT,
    dosage_form TEXT,
    spl_id TEXT,
    product_type TEXT,
    marketing_start_date TEXT,
    product_id TEXT,
    application_number TEXT NULL,
    brand_name_base TEXT NULL
)
