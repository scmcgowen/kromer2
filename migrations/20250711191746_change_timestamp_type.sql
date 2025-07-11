-- Change TIMESTAMP fields to TIMESTAMPTZ for timezone awareness

-- Update wallets table
ALTER TABLE wallets ALTER COLUMN created_at TYPE TIMESTAMPTZ;

-- Update transactions table
ALTER TABLE transactions ALTER COLUMN date TYPE TIMESTAMPTZ;

-- Update names table
ALTER TABLE names ALTER COLUMN time_registered TYPE TIMESTAMPTZ;
ALTER TABLE names ALTER COLUMN last_transfered TYPE TIMESTAMPTZ;
ALTER TABLE names ALTER COLUMN last_updated TYPE TIMESTAMPTZ;
