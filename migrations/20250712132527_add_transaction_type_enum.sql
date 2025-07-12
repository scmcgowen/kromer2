-- Add a transaction type enum
CREATE TYPE transaction_type AS ENUM ('mined', 'unknown', 'name_purchase', 'name_a_record', 'name_transfer', 'transfer');

-- Update the transactions table to use the new enum type
ALTER TABLE transactions ALTER COLUMN transaction_type TYPE transaction_type USING transaction_type::text::transaction_type;
