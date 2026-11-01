use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, strum::IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum ArticleStatus {
    Draft,
    Published,
}
