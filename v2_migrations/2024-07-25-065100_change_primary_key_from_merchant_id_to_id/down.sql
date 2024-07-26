-- This file should undo anything in `up.sql`
-- The new primary key for v2 merchant account will be `id`
ALTER TABLE merchant_account DROP CONSTRAINT merchant_account_pkey;

-- In order to run this query, the merchant_id column should be unique and not null
-- We need to backfill the id, a simple strategy will be to copy the values of id to merchant_id
-- Query to update the merchant_id column with values of id
UPDATE merchant_account
SET merchant_id = id;

-- Note: This command might not run successfully for the existing table
-- This is because there will be some rows ( which are created via v2 application ) which will have id as empty
-- A backfill might be required to run this query
-- However if this is being run on a fresh database, this should succeed
ALTER TABLE merchant_account
ADD PRIMARY KEY (merchant_id);
