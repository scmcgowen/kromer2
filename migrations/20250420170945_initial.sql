-- ------------------------------
-- TABLE: wallets
-- ------------------------------
CREATE TABLE `wallets` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `address` CHAR(10) NOT NULL,
    `balance` DECIMAL(16, 2) NOT NULL DEFAULT 0.00,
    `created_at` TIMESTAMP NOT NULL,
    `locked` TINYINT (1) DEFAULT 0.00,
    `total_in` DECIMAL(16, 2) NOT NULL DEFAULT 0.00,
    `total_out` DECIMAL(16, 2) NOT NULL DEFAULT 0.00,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `idx_wallet_address` (`address`)
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ------------------------------
-- TABLE: transactions
-- ------------------------------
CREATE TABLE `transactions` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `amount` DECIMAL(16, 2) NOT NULL,
    `from` CHAR(10) NULL,
    `to` CHAR(10) NOT NULL,
    `metadata` VARCHAR(512) NULL,
    `transaction_type` ENUM ('unknown', 'mined', 'name_purchase', 'transfer'),
    `date` TIMESTAMP NOT NULL,
    PRIMARY KEY (`id`)
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ------------------------------
-- TABLE: names
-- ------------------------------
CREATE TABLE `names` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(64) NOT NULL,
    `owner` CHAR(10) NOT NULL,
    `original_owner` CHAR(10) NOT NULL,
    `time_registered` TIMESTAMP NOT NULL,
    `last_transfered` TIMESTAMP NULL,
    `last_updated` TIMESTAMP NULL,
    `unpaid` DECIMAL(16, 2) NOT NULL DEFAULT 0,
    `metadata` VARCHAR(255) NULL,
    PRIMARY KEY (`id`)
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;

-- ------------------------------
-- TABLE: players
-- ------------------------------
CREATE TABLE `players` (
    `id` UUID NOT NULL,
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`id`)
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
