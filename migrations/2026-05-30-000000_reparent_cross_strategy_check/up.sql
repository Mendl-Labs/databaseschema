-- Tighten reparent_backtest_job to reject cross-strategy reparents.
--
-- backtest_jobs.strategy_type is the closest proxy we have for "is this the
-- same family of strategy?" (there is no strategy_id FK on backtest_jobs).
-- Allowing a momentum job to be reparented onto a mean-reversion lineage
-- corrupts the meaning of the tree, so we refuse it at the SQL choke point.
--
-- This is a forward-only redefinition of the existing function. Cycle and
-- same-tenant checks are preserved verbatim.

CREATE OR REPLACE FUNCTION reparent_backtest_job(
    p_job_id        UUID,
    p_new_parent_id UUID
) RETURNS UUID
LANGUAGE plpgsql AS $$
DECLARE
    v_job_tenant     UUID;
    v_job_strategy   VARCHAR(50);
    v_parent_tenant  UUID;
    v_parent_root    UUID;
    v_parent_strategy VARCHAR(50);
    v_cur            UUID;
    v_depth          INT := 0;
    v_max_depth      CONSTANT INT := 200;
    v_new_root       UUID;
BEGIN
    IF p_job_id IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: p_job_id cannot be null';
    END IF;
    IF p_new_parent_id IS NOT NULL AND p_new_parent_id = p_job_id THEN
        RAISE EXCEPTION 'reparent_backtest_job: a job cannot be its own parent (%)', p_job_id;
    END IF;

    SELECT tenant_id, strategy_type
      INTO v_job_tenant, v_job_strategy
      FROM backtest_jobs
     WHERE id = p_job_id
       FOR UPDATE;
    IF v_job_tenant IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: job % not found', p_job_id;
    END IF;

    -- Promote to root.
    IF p_new_parent_id IS NULL THEN
        UPDATE backtest_jobs
           SET parent_job_id = NULL,
               root_job_id   = id
         WHERE id = p_job_id;
        PERFORM _recompute_lineage_subtree(p_job_id);
        RETURN p_job_id;
    END IF;

    SELECT tenant_id, root_job_id, strategy_type
      INTO v_parent_tenant, v_parent_root, v_parent_strategy
      FROM backtest_jobs
     WHERE id = p_new_parent_id;
    IF v_parent_tenant IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: new parent % not found', p_new_parent_id;
    END IF;
    IF v_parent_tenant <> v_job_tenant THEN
        RAISE EXCEPTION 'reparent_backtest_job: cross-tenant reparent denied (job tenant=%, parent tenant=%)',
            v_job_tenant, v_parent_tenant;
    END IF;

    -- NEW: cross-strategy guard. Lineage trees must stay within one
    -- strategy_type family so the resulting tree has a coherent meaning.
    IF v_parent_strategy IS DISTINCT FROM v_job_strategy THEN
        RAISE EXCEPTION 'reparent_backtest_job: cross-strategy reparent denied (job=%, parent=%)',
            v_job_strategy, v_parent_strategy;
    END IF;

    -- Cycle check.
    v_cur := p_new_parent_id;
    WHILE v_cur IS NOT NULL AND v_depth < v_max_depth LOOP
        IF v_cur = p_job_id THEN
            RAISE EXCEPTION 'reparent_backtest_job: would create cycle (new parent % is a descendant of %)',
                p_new_parent_id, p_job_id;
        END IF;
        SELECT parent_job_id INTO v_cur FROM backtest_jobs WHERE id = v_cur;
        v_depth := v_depth + 1;
    END LOOP;
    IF v_depth >= v_max_depth THEN
        RAISE EXCEPTION 'reparent_backtest_job: lineage too deep (>% levels) — refusing to reparent', v_max_depth;
    END IF;

    v_new_root := COALESCE(v_parent_root, p_new_parent_id);

    UPDATE backtest_jobs
       SET parent_job_id = p_new_parent_id,
           root_job_id   = v_new_root
     WHERE id = p_job_id;

    PERFORM _recompute_lineage_subtree(p_job_id);
    RETURN v_new_root;
END;
$$;

COMMENT ON FUNCTION reparent_backtest_job(UUID, UUID)
    IS 'Single choke point for mutating run lineage. Enforces no cycles, same-tenant, same strategy_type, bounded depth. Returns the resulting root_job_id. Pass NULL as new_parent to promote to root.';
