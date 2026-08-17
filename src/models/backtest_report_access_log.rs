use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_report_access_log)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BacktestReportAccessLog {
    pub id: Uuid,
    pub report_id: Uuid,
    pub accessed_by: Option<String>,
    pub access_method: String,
    pub format_requested: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub response_time_ms: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
    pub accessed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_report_access_log)]
pub struct NewBacktestReportAccessLog {
    pub report_id: Uuid,
    pub accessed_by: Option<String>,
    pub access_method: String,
    pub format_requested: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub response_time_ms: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
}
