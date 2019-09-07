mod connection;
mod connection_visibility;
mod current_user;
mod flags;
mod premium_type;
mod profile;

pub use self::{
    connection::Connection,
    connection_visibility::ConnectionVisibility,
    current_user::CurrentUser,
    flags::UserFlags,
    premium_type::PremiumType,
    profile::UserProfile,
};

use crate::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    #[serde(rename = "username")]
    pub name: String,
}