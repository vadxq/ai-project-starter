use rust_axum_starter::platform::db::sql::{mark_used_sql, soft_delete_sql};

#[test]
fn soft_delete_sql_builds_expected_statement() {
    let sql = soft_delete_sql("sample_items", "id = $1");
    assert_eq!(
        sql,
        "UPDATE sample_items SET deleted_at = now(), updated_at = now() WHERE id = $1 AND deleted_at IS NULL"
    );
}

#[test]
fn mark_used_sql_builds_expected_statement() {
    let sql = mark_used_sql("sample_jobs", "id = $1");
    assert_eq!(
        sql,
        "UPDATE sample_jobs SET used_at = now(), updated_at = now() WHERE id = $1 AND used_at IS NULL AND deleted_at IS NULL"
    );
}

#[test]
fn mysql_sql_uses_question_mark_placeholders() {
    assert_eq!(
        soft_delete_sql("sample_items", "id = ?"),
        "UPDATE sample_items SET deleted_at = now(), updated_at = now() WHERE id = ? AND deleted_at IS NULL"
    );
    assert_eq!(
        mark_used_sql("sample_jobs", "id = ?"),
        "UPDATE sample_jobs SET used_at = now(), updated_at = now() WHERE id = ? AND used_at IS NULL AND deleted_at IS NULL"
    );
}
