CREATE TABLE IF NOT EXISTS product (
    product_ndc VARCHAR(20) PRIMARY KEY,
    generic_name VARCHAR(255),
    labeler_name VARCHAR(255) NOT NULL,
    brand_name VARCHAR(255),
    finished BOOLEAN NOT NULL,
    listing_expiration_date DATE,
    marketing_category VARCHAR(255) NOT NULL,
    dosage_form VARCHAR(255) NOT NULL,
    product_type VARCHAR(255) NOT NULL,
    marketing_start_date DATE NOT NULL,
    application_number VARCHAR(255),
    brand_name_base VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS package (
    package_ndc VARCHAR(20) PRIMARY KEY,
    product_ndc INT REFERENCES product(product_ndc) ON DELETE CASCADE,
    description TEXT NOT NULL,
    marketing_start_date DATE NOT NULL,
    sample BOOLEAN
);

CREATE TABLE IF NOT EXISTS active_ingredient (
    id SERIAL PRIMARY KEY,
    product_ndc VARCHAR(20) REFERENCES product(product_ndc) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    strength VARCHAR(50)
);
