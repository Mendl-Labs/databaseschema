-- Revert to the previous reparent_backtest_job (without cross-strategy check).
-- See 2026-05-29-000000_add_run_lineage/up.sql for the original definition.

CREATE OR REPLACE FUNCTION reparent_backtest_job(
    p_job_id        UUID,
    p_new_parent_id UUID
) RETURNS UUID
LANGUAGE plpgsql AS $$
DECLARE
    v_job_tenant    UUID;
    v_parent_tenant UUID;
    v_parent_root   UUID;
    v_cur           UUID;
    v_depth         INT := 0;
    v_max_depth     CONSTANT INT := 200;
    v_new_root      UUID;
BEGIN
    IF p_job_id IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: p_job_id cannot be null';
    END IF;
    IF p_new_parent_id IS NOT NULL AND p_new_parent_id = p_job_id THEN
        RAISE EXCEPTION 'reparent_backtest_job: a job cannot be its own parent (%)', p_job_id;
    END IF;

    SELECT tenant_id INTO v_job_tenant FROM backtest_jobs WHERE id = p_job_id FOR UPDATE;
    IF v_job_tenant IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: job % not found', p_job_id;
    END IF;

    IF p_new_parent_id IS NULL THEN
        UPDATE backtest_jobs SET parent_job_id = NULL, root_job_id = id WHERE id = p_job_id;
        PERFORM _recompute_lineage_subtree(p_job_id);
        RETURN p_job_id;
    END IF;

    SELECT tenant_id, root_job_id INTO v_parent_tenant, v_parent_root
      FROM backtest_jobs WHERE id = p_new_parent_id;
    IF v_parent_tenant IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: new parent % not found', p_new_parent_id;
    END IF;
    IF v_parent_tenant <> v_job_tenant THEN
        RAISE EXCEPTION 'reparent_backtest_job: cross-tenant reparent denied';
    END IF;

    v_cur := p_new_parent_id;
    WHILE v_cur IS NOT NULL AND v_depth < v_max_depth LOOP
        IF v_cur = p_job_id THEN
            RAISE EXCEPTION 'reparent_backtest_job: would create cycle';
        END IF;
        SELECT parent_job_id INTO v_cur FROM backtest_jobs WHERE id = v_cur;
        v_depth := v_depth + 1;
    END LOOP;
    IF v_depth >= v_max_depth THEN
        RAISE EXCEPTION 'reparent_backtest_job: lineage too deep';
    END IF;

    v_new_root := COALESCE(v_parent_root, p_new_parent_id);

    UPDATE backtest_jobs SET parent_job_id = p_new_parent_id, root_job_id = v_new_root WHERE id = p_job_id;
    PERFORM _recompute_lineage_subtree(p_job_id);
    RETURN v_new_root;
END;
$$;
