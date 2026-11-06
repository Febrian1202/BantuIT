use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, IntoStaticStr)]
pub enum RateLimitTier {
    Login,
    Upload,
    Search,
    Export,
    Api,
}
