-- Run lineage: parent/root pointers, content hashes, hypothesis text,
-- companion notes table, and a single reparent function as the choke point.
--
-- Design goals:
--   * Forward-only, backfill-safe (all new columns nullable, existing rows
--     backfilled so root_job_id = id — i.e. every existing job becomes its
--     own root).
--   * Single SQL choke point (reparent_backtest_job) so app code cannot
--     write inconsistent lineage. Enforces no cycles and same-tenant.
--   * Cheap subtree queries via cached root_job_id; cheap parent walk
--     via indexed parent_job_id.

-- ---------------------------------------------------------------------------
-- 1. Schema additions on backtest_jobs
-- ---------------------------------------------------------------------------
ALTER TABLE backtest_jobs
    ADD COLUMN parent_job_id UUID NULL REFERENCES backtest_jobs(id) ON DELETE SET NULL,
    ADD COLUMN root_job_id   UUID NULL REFERENCES backtest_jobs(id) ON DELETE SET NULL,
    ADD COLUMN code_hash     TEXT NULL,
    ADD COLUMN params_hash   TEXT NULL,
    ADD COLUMN hypothesis    TEXT NULL;

-- Backfill: every existing job is its own root (lineage starts here).
UPDATE backtest_jobs SET root_job_id = id WHERE root_job_id IS NULL;

-- Lineage shape invariant: a row is either a root (parent NULL, root = id or NULL)
-- or a child (parent NOT NULL, root NOT NULL, root != id).
ALTER TABLE backtest_jobs ADD CONSTRAINT lineage_root_consistent
    CHECK (
        (parent_job_id IS NULL AND (root_job_id IS NULL OR root_job_id = id))
        OR
        (parent_job_id IS NOT NULL AND root_job_id IS NOT NULL AND root_job_id <> id)
    );

-- Hash format: lowercase hex sha256 (64 chars) — enforce length only, not content.
ALTER TABLE backtest_jobs ADD CONSTRAINT code_hash_format
    CHECK (code_hash IS NULL OR length(code_hash) = 64);
ALTER TABLE backtest_jobs ADD CONSTRAINT params_hash_format
    CHECK (params_hash IS NULL OR length(params_hash) = 64);

-- Indexes
CREATE INDEX idx_backtest_jobs_parent       ON backtest_jobs(parent_job_id)              WHERE parent_job_id IS NOT NULL;
CREATE INDEX idx_backtest_jobs_root         ON backtest_jobs(root_job_id)                WHERE root_job_id   IS NOT NULL;
CREATE INDEX idx_backtest_jobs_code_hash    ON backtest_jobs(tenant_id, code_hash)       WHERE code_hash     IS NOT NULL;
CREATE INDEX idx_backtest_jobs_params_hash  ON backtest_jobs(tenant_id, params_hash)     WHERE params_hash   IS NOT NULL;

COMMENT ON COLUMN backtest_jobs.parent_job_id IS 'The job that inspired/preceded this one (lineage edge). NULL = root of its branch.';
COMMENT ON COLUMN backtest_jobs.root_job_id   IS 'Cached root of this lineage tree for cheap subtree queries. Equals id for roots.';
COMMENT ON COLUMN backtest_jobs.code_hash     IS 'sha256 (lowercase hex) of strategy source/config — for detecting code-vs-params iterations.';
COMMENT ON COLUMN backtest_jobs.params_hash   IS 'sha256 (lowercase hex) of params_json — for detecting param-only iterations.';
COMMENT ON COLUMN backtest_jobs.hypothesis    IS 'User-supplied one-liner about what they expect from this run.';

-- ---------------------------------------------------------------------------
-- 2. run_lineage_notes: free-form narrative attached to a run.
-- ---------------------------------------------------------------------------
CREATE TABLE run_lineage_notes (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id      UUID NOT NULL REFERENCES backtest_jobs(id) ON DELETE CASCADE,
    tenant_id   UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    note_type   VARCHAR(32) NOT NULL DEFAULT 'observation',
    body        TEXT NOT NULL,
    author      VARCHAR(64) NULL,   -- clerk user id, or NULL for system/AI-generated
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_note_type CHECK (note_type IN ('hypothesis', 'observation', 'verdict', 'next_step', 'ai_summary'))
);
CREATE INDEX idx_run_lineage_notes_job    ON run_lineage_notes(job_id, created_at);
CREATE INDEX idx_run_lineage_notes_tenant ON run_lineage_notes(tenant_id, created_at DESC);

COMMENT ON TABLE  run_lineage_notes IS 'Per-run narrative: hypotheses, observations, verdicts, next-step plans. Append-only.';
COMMENT ON COLUMN run_lineage_notes.note_type IS 'hypothesis | observation | verdict | next_step | ai_summary';
COMMENT ON COLUMN run_lineage_notes.author    IS 'Clerk user id when user-authored, NULL when system/AI-generated.';

-- ---------------------------------------------------------------------------
-- 3. Subtree root recomputation (internal helper, prefixed _).
-- ---------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION _recompute_lineage_subtree(p_root UUID)
RETURNS INTEGER
LANGUAGE plpgsql AS $$
DECLARE
    v_new_root UUID;
    v_updated  INTEGER;
BEGIN
    SELECT root_job_id INTO v_new_root FROM backtest_jobs WHERE id = p_root;
    IF v_new_root IS NULL THEN
        RAISE EXCEPTION '_recompute_lineage_subtree: root job % not found', p_root;
    END IF;

    WITH RECURSIVE subtree AS (
        SELECT id FROM backtest_jobs WHERE parent_job_id = p_root
        UNION ALL
        SELECT bj.id
          FROM backtest_jobs bj
          JOIN subtree st ON bj.parent_job_id = st.id
    )
    UPDATE backtest_jobs
       SET root_job_id = v_new_root
     WHERE id IN (SELECT id FROM subtree);

    GET DIAGNOSTICS v_updated = ROW_COUNT;
    RETURN v_updated;
END;
$$;

COMMENT ON FUNCTION _recompute_lineage_subtree(UUID)
    IS 'Internal helper. Walks the descendant subtree of p_root and refreshes their cached root_job_id. Called by reparent_backtest_job.';

-- ---------------------------------------------------------------------------
-- 4. reparent_backtest_job: THE single mutation surface for lineage edges.
-- ---------------------------------------------------------------------------
--   * Cycle-safe (walks up from new parent; rejects if we encounter p_job_id).
--   * Tenant-scoped (refuses cross-tenant reparent).
--   * Promotes to root when p_new_parent_id IS NULL.
--   * Propagates new root_job_id to the moved subtree.
--   * Bounded walk (200 levels) so a corrupted lineage cannot hang the txn.
CREATE OR REPLACE FUNCTION reparent_backtest_job(
    p_job_id        UUID,
    p_new_parent_id UUID
) RETURNS UUID  -- the resulting root_job_id
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

    -- Lock the row we're mutating so concurrent reparents can't race.
    SELECT tenant_id
      INTO v_job_tenant
      FROM backtest_jobs
     WHERE id = p_job_id
       FOR UPDATE;
    IF v_job_tenant IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: job % not found', p_job_id;
    END IF;

    -- Case A: promote to root.
    IF p_new_parent_id IS NULL THEN
        UPDATE backtest_jobs
           SET parent_job_id = NULL,
               root_job_id   = id
         WHERE id = p_job_id;
        PERFORM _recompute_lineage_subtree(p_job_id);
        RETURN p_job_id;
    END IF;

    -- Case B: attach to an existing parent.
    SELECT tenant_id, root_job_id
      INTO v_parent_tenant, v_parent_root
      FROM backtest_jobs
     WHERE id = p_new_parent_id;
    IF v_parent_tenant IS NULL THEN
        RAISE EXCEPTION 'reparent_backtest_job: new parent % not found', p_new_parent_id;
    END IF;
    IF v_parent_tenant <> v_job_tenant THEN
        RAISE EXCEPTION 'reparent_backtest_job: cross-tenant reparent denied (job tenant=%, parent tenant=%)',
            v_job_tenant, v_parent_tenant;
    END IF;

    -- Cycle check: walk up the parent chain from the proposed new parent.
    -- If we encounter p_job_id we'd create a loop.
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

    -- The new root is the parent's root (or the parent itself if it is a root).
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
    IS 'Single choke point for mutating run lineage. Enforces no cycles, same-tenant, bounded depth. Returns the resulting root_job_id. Pass NULL as new_parent to promote to root.';
