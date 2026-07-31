-- Pairs-trading live deployments: tag sibling-leg position rows so risk
-- checks can net a hedged pair's exposure instead of summing it as gross
-- (a $10k long-A / $10k short-B pair should read as ~$0 net delta, not
-- $20k of exposure against max_position_size/max_total_exposure).
--
-- Nullable, no default: every existing (and every ordinary non-pair) row
-- stays untagged and continues to be treated as gross/independent exposure
-- exactly as before -- this is purely additive.

ALTER TABLE deployment_positions
    ADD COLUMN pair_group_id UUID,
    ADD COLUMN leg_role VARCHAR(16);

ALTER TABLE deployment_positions
    ADD CONSTRAINT deployment_positions_leg_role_check
    CHECK (leg_role IS NULL OR leg_role IN ('pair_long', 'pair_short'));

-- Fast lookup of a pair's sibling leg (used by the netted-exposure risk
-- check to find "this row's other leg" without scanning every position for
-- the deployment).
CREATE INDEX idx_deployment_positions_pair_group
    ON deployment_positions (pair_group_id)
    WHERE pair_group_id IS NOT NULL;

COMMENT ON COLUMN deployment_positions.pair_group_id IS 'Links two sibling-leg rows of one pairs-trading position under the same deployment. NULL for every ordinary (non-pair) position.';
COMMENT ON COLUMN deployment_positions.leg_role IS 'pair_long or pair_short -- which side of the pair this row is. NULL when pair_group_id is NULL.';
