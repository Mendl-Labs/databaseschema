-- Add deployment mode and cooldown to deployed_strategies
-- mode: 'paper' (simulated) or 'live' (real exchange orders), default paper for safety
-- cooldown_minutes: minimum wait between trades for risk management
ALTER TABLE deployed_strategies ADD COLUMN mode VARCHAR(20) NOT NULL DEFAULT 'paper';
ALTER TABLE deployed_strategies ADD COLUMN cooldown_minutes INTEGER;
