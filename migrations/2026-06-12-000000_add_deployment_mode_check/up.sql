-- Constrain deployed_strategies.mode to known values.
-- Normalize any stray values to 'paper' (the safe default) before adding the constraint.
UPDATE deployed_strategies SET mode = 'paper' WHERE mode NOT IN ('paper', 'live');

ALTER TABLE deployed_strategies
    ADD CONSTRAINT deployed_strategies_mode_check CHECK (mode IN ('paper', 'live'));
