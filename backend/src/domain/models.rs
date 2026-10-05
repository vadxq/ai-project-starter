use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize, Serializer};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub const MAX_TITLE_BYTES: usize = 240;
pub const MAX_PAGE_SIZE: i64 = 100;

fn timestamp<S: Serializer>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_rfc3339_opts(SecondsFormat::Millis, true))
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub version: i32,
    #[serde(serialize_with = "timestamp")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "timestamp")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateItem {
    #[schema(min_length = 1, max_length = 240)]
    pub title: String,
    pub completed: bool,
}

// 缺少字段表示不修改；显式 null 必须被拒绝，防止生成 SDK 隐式清空值。
fn non_null<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateItem {
    #[schema(minimum = 1)]
    pub version: i32,
    #[serde(
        default,
        deserialize_with = "non_null",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(nullable = false, min_length = 1, max_length = 240)]
    pub title: Option<String>,
    #[serde(
        default,
        deserialize_with = "non_null",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(nullable = false)]
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    #[param(minimum = 1, maximum = 100)]
    pub limit: i64,
    #[param(minimum = 0)]
    pub offset: i64,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct DeleteVersion {
    #[param(minimum = 1)]
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ItemPage {
    pub items: Vec<Item>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Health {
    pub status: String,
}

pub fn normalize_title(title: &str) -> Option<String> {
    let normalized = title.trim_matches([' ', '\t', '\r', '\n']);
    (1..=MAX_TITLE_BYTES)
        .contains(&normalized.len())
        .then(|| normalized.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_uses_utf8_bytes_and_ascii_trimming() {
        assert_eq!(normalize_title(" \t任务\r\n"), Some("任务".to_owned()));
        assert!(normalize_title(&"中".repeat(80)).is_some());
        assert!(normalize_title(&"中".repeat(81)).is_none());
        assert!(normalize_title(" \t\n").is_none());
        assert_eq!(normalize_title("\u{a0}"), Some("\u{a0}".to_owned()));
    }

    #[test]
    fn patch_rejects_null_and_unknown_fields() {
        assert!(serde_json::from_str::<UpdateItem>(r#"{"version":1,"title":null}"#).is_err());
        assert!(serde_json::from_str::<UpdateItem>(r#"{"version":1,"completed":null}"#).is_err());
        assert!(serde_json::from_str::<UpdateItem>(r#"{"version":1,"extra":true}"#).is_err());
        let patch =
            serde_json::from_str::<UpdateItem>(r#"{"version":1,"completed":false}"#).unwrap();
        assert_eq!(patch.completed, Some(false));
        assert!(patch.title.is_none());
    }
}
