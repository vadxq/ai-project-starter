//! 生成软删除和消费状态场景复用的 SQL 片段。

pub const ACTIVE_FILTER: &str = "deleted_at IS NULL";
pub const UNUSED_FILTER: &str = "used_at IS NULL";
pub const SOFT_DELETE_SET: &str = "deleted_at = now(), updated_at = now()";
pub const MARK_USED_SET: &str = "used_at = now(), updated_at = now()";

fn update_sql(table: &str, set_clause: &str, predicate: &str) -> String {
    format!("UPDATE {table} SET {set_clause} WHERE {predicate}")
}

fn active_predicate(predicate: &str) -> String {
    join_predicates(&[predicate, ACTIVE_FILTER])
}

fn unused_active_predicate(predicate: &str) -> String {
    join_predicates(&[predicate, UNUSED_FILTER, ACTIVE_FILTER])
}

/// 生成仅作用于未删除记录的软删除语句。
pub fn soft_delete_sql(table: &str, predicate: &str) -> String {
    let predicate = active_predicate(predicate);
    update_sql(table, SOFT_DELETE_SET, &predicate)
}

/// 生成仅作用于未消费且未删除记录的“标记已使用”语句。
pub fn mark_used_sql(table: &str, predicate: &str) -> String {
    let predicate = unused_active_predicate(predicate);
    update_sql(table, MARK_USED_SET, &predicate)
}

fn join_predicates(predicates: &[&str]) -> String {
    predicates
        .iter()
        .map(|predicate| predicate.trim())
        .filter(|predicate| !predicate.is_empty())
        .collect::<Vec<_>>()
        .join(" AND ")
}
