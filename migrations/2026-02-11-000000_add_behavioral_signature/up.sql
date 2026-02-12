-- Add behavioral signature columns to deployed_strategies
-- Used for strategy crowding prevention / diversity preservation
-- Tracks how strategies behave to penalize similar strategies during GA optimization

-- Behavioral signature captures trading behavior (not just parameters):
-- - hold_time_bucket, frequency_bucket, signal_type, sizing_style
-- - active_hours bitmask, risk_profile, reward_risk_bucket
ALTER TABLE deployed_strategies
ADD COLUMN behavioral_signature JSONB;

-- Parameter hash for quick similarity detection
-- 64-bit hash of key strategy parameters (quantized)
ALTER TABLE deployed_strategies
ADD COLUMN parameter_hash BIGINT;

-- AUM tracking for capacity-weighted crowding
-- Updated by SignalEngine when capital allocation changes
ALTER TABLE deployed_strategies
ADD COLUMN current_aum DECIMAL(20, 8) DEFAULT 0;

-- Index for crowding queries (find similar strategies)
CREATE INDEX idx_deployed_strategies_param_hash ON deployed_strategies(parameter_hash) 
WHERE is_active = true;

-- Partial index for active strategies with behavioral signatures
CREATE INDEX idx_deployed_strategies_behavioral ON deployed_strategies(id) 
WHERE is_active = true AND behavioral_signature IS NOT NULL;

-- Comment for documentation
COMMENT ON COLUMN deployed_strategies.behavioral_signature IS 'JSON behavioral profile for diversity/crowding detection';
COMMENT ON COLUMN deployed_strategies.parameter_hash IS '64-bit hash of key parameters for quick similarity check';
COMMENT ON COLUMN deployed_strategies.current_aum IS 'Current AUM in USD for capacity-weighted crowding penalties';
