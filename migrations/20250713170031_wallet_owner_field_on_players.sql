ALTER TABLE players ADD COLUMN owned_wallets INTEGER[];
ALTER TABLE players
    ALTER COLUMN owned_wallets set default array[]::INTEGER[];
