-- Reverse: restore risk_tolerance and preferred_execution_tier, drop new columns
ALTER TABLE ai_user_profiles
    ADD COLUMN risk_tolerance VARCHAR(50) DEFAULT 'moderate',
    ADD COLUMN preferred_execution_tier INT DEFAULT 2;

UPDATE ai_user_profiles
SET risk_tolerance = CASE
    WHEN preferred_fitness_preset = 'conservative' THEN 'conservative'
    WHEN preferred_fitness_preset = 'balanced'     THEN 'moderate'
    WHEN preferred_fitness_preset = 'aggressive'   THEN 'aggressive'
    ELSE 'moderate'
END;

ALTER TABLE ai_user_profiles
    DROP COLUMN preferred_fitness_preset,
    DROP COLUMN preferred_exchanges,
    DROP COLUMN preferred_capital_range;
