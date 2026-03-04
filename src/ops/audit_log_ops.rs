//! Audit Log Database Operations
//!
//! CRUD operations for audit_logs table — compliance and security tracking.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::audit_log::{AuditLog, NewAuditLog};
use crate::schema::audit_logs;

/// Insert a new audit log entry
pub async fn insert_audit_log(
    conn: &mut AsyncPgConnection,
    log: NewAuditLog,
) -> Result<AuditLog, diesel::result::Error> {
    diesel::insert_into(audit_logs::table)
        .values(&log)
        .returning(AuditLog::as_returning())
        .get_result(conn)
        .await
}

/// Insert a batch of audit log entries
pub async fn insert_audit_logs_batch(
    conn: &mut AsyncPgConnection,
    logs: Vec<NewAuditLog>,
) -> Result<usize, diesel::result::Error> {
    if logs.is_empty() {
        return Ok(0);
    }

    diesel::insert_into(audit_logs::table)
        .values(&logs)
        .execute(conn)
        .await
}

/// Get audit logs for a tenant, ordered by most recent first
pub async fn get_audit_logs_for_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<AuditLog>, diesel::result::Error> {
    audit_logs::table
        .filter(audit_logs::tenant_id.eq(tenant_id))
        .select(AuditLog::as_select())
        .order(audit_logs::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load(conn)
        .await
}

/// Get audit logs for a tenant within a time range
pub async fn get_audit_logs_in_range(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    action_type: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<AuditLog>, diesel::result::Error> {
    let mut query = audit_logs::table
        .select(AuditLog::as_select())
        .filter(audit_logs::tenant_id.eq(tenant_id))
        .filter(audit_logs::created_at.ge(start))
        .filter(audit_logs::created_at.lt(end))
        .order(audit_logs::created_at.desc())
        .into_boxed();

    if let Some(at) = action_type {
        query = query.filter(audit_logs::action_type.eq(at));
    }
    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    query.load(conn).await
}

/// Get audit logs for a specific resource
pub async fn get_audit_logs_for_resource(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    resource_type: &str,
    resource_id: Uuid,
) -> Result<Vec<AuditLog>, diesel::result::Error> {
    audit_logs::table
        .filter(audit_logs::tenant_id.eq(tenant_id))
        .filter(audit_logs::resource_type.eq(resource_type))
        .filter(audit_logs::resource_id.eq(resource_id))
        .select(AuditLog::as_select())
        .order(audit_logs::created_at.desc())
        .load(conn)
        .await
}

/// Get audit logs for a specific user
pub async fn get_audit_logs_for_user(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<AuditLog>, diesel::result::Error> {
    audit_logs::table
        .filter(audit_logs::tenant_id.eq(tenant_id))
        .filter(audit_logs::user_id.eq(user_id))
        .select(AuditLog::as_select())
        .order(audit_logs::created_at.desc())
        .limit(limit)
        .load(conn)
        .await
}

/// Count audit logs for a tenant (for pagination)
pub async fn count_audit_logs_for_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<i64, diesel::result::Error> {
    audit_logs::table
        .filter(audit_logs::tenant_id.eq(tenant_id))
        .count()
        .get_result(conn)
        .await
}
