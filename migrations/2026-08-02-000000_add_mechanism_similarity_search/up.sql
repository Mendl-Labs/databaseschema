-- Enables trigram-based free-text similarity search over
-- backtest_jobs.params_json->>'strategy_mechanism', used by the
-- get_similar_edge_mechanisms AI tool to find a tenant's own past
-- hypotheses that describe the same underlying edge in different words --
-- something exact edge_type tag matching (get_edge_type_track_record /
-- get_edge_type_decay) cannot do.
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Expression GIN index on the extracted JSONB text field so similarity()
-- lookups don't require a sequential scan as backtest_jobs grows.
CREATE INDEX IF NOT EXISTS idx_backtest_jobs_mechanism_trgm
    ON backtest_jobs
    USING GIN ((params_json->>'strategy_mechanism') gin_trgm_ops);
