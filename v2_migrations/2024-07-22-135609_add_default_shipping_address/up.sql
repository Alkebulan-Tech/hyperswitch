-- Your SQL goes here
ALTER TABLE customers ADD COLUMN IF NOT EXISTS default_shipping_address JSONB;