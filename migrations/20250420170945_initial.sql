-- ------------------------------
-- TABLE: wallets
-- ------------------------------
CREATE TABLE wallets (
    id SERIAL PRIMARY KEY,
    address CHAR(10) NOT NULL,
    balance NUMERIC(16, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMP NOT NULL,
    locked BOOLEAN DEFAULT FALSE,
    total_in NUMERIC(16, 2) NOT NULL DEFAULT 0.00,
    total_out NUMERIC(16, 2) NOT NULL DEFAULT 0.00
);

CREATE UNIQUE INDEX idx_wallet_address ON wallets (address);

-- ------------------------------
-- TABLE: transactions
-- ------------------------------
CREATE TYPE transaction_type_enum AS ENUM ('unknown', 'mined', 'name_purchase', 'transfer');

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    amount NUMERIC(16, 2) NOT NULL,
    "from" CHAR(10) NULL,
    "to" CHAR(10) NOT NULL,
    metadata VARCHAR(512) NULL,
    transaction_type transaction_type_enum,
    date TIMESTAMP NOT NULL
);

-- ------------------------------
-- TABLE: names
-- ------------------------------
CREATE TABLE names (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    owner CHAR(10) NOT NULL,
    original_owner CHAR(10) NOT NULL,
    time_registered TIMESTAMP NOT NULL,
    last_transfered TIMESTAMP NULL,
    last_updated TIMESTAMP NULL,
    unpaid NUMERIC(16, 2) NOT NULL DEFAULT 0,
    metadata VARCHAR(255) NULL
);

-- ------------------------------
-- TABLE: players
-- ------------------------------
CREATE TABLE players (
    id UUID PRIMARY KEY,
    name VARCHAR(16) NOT NULL
);
