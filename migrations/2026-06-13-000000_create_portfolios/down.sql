DROP TRIGGER IF EXISTS trigger_portfolios_updated_at ON portfolios;
DROP FUNCTION IF EXISTS update_portfolios_updated_at();

DROP INDEX IF EXISTS idx_portfolios_tenant_name;
DROP INDEX IF EXISTS idx_backtest_results_portfolio;
ALTER TABLE backtest_results DROP COLUMN IF EXISTS portfolio_id;

DROP INDEX IF EXISTS idx_portfolio_assets_portfolio;
DROP INDEX IF EXISTS idx_portfolios_tenant;
DROP INDEX IF EXISTS idx_portfolio_assets_unique;

DROP TABLE IF EXISTS portfolio_assets;
DROP TABLE IF EXISTS portfolios;
