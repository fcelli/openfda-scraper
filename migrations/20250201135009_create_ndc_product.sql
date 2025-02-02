CREATE TABLE IF NOT EXISTS ndc_product (
    product_ndc VARCHAR(9) PRIMARY KEY,
    generic_name VARCHAR(255) NULL,
    labeler_name VARCHAR(255),
    brand_name VARCHAR(255) NULL,
    finished BOOLEAN,
    listing_expiration_date DATE NULL,
    marketing_category VARCHAR(255),
    dosage_form VARCHAR(255),
    product_type VARCHAR(255),
    marketing_start_date DATE,
    application_number VARCHAR(255) NULL,
    brand_name_base VARCHAR(255) NULL
)
