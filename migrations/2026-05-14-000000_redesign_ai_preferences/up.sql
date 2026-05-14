-- Redesign ai_user_profiles: remove execution tier, rename risk_tolerance → preferred_fitness_preset,
-- add preferred_exchanges and preferred_capital_range columns.

-- Step 1: Add new columns
ALTER TABLE ai_user_profiles
    ADD COLUMN preferred_fitness_preset VARCHAR(50),
    ADD COLUMN preferred_exchanges JSONB DEFAULT '[]'::JSONB,
    ADD COLUMN preferred_capital_range JSONB;

-- Step 2: Migrate risk_tolerance → preferred_fitness_preset
-- conservative→conservative, moderate→balanced, aggressive→aggressive; NULL→NULL
UPDATE ai_user_profiles
SET preferred_fitness_preset = CASE
    WHEN risk_tolerance = 'conservative' THEN 'conservative'
    WHEN risk_tolerance = 'moderate'     THEN 'balanced'
    WHEN risk_tolerance = 'aggressive'   THEN 'aggressive'
    ELSE NULL
END;

-- Step 3: Drop old columns
ALTER TABLE ai_user_profiles
    DROP COLUMN risk_tolerance,
    DROP COLUMN preferred_execution_tier;
