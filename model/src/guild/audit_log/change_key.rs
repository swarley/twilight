use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditLogChangeKey {
    AfkChannelId,
    AfkTimeout,
    Allow,
    ApplicationId,
    AvatarHash,
    Bitrate,
    ChannelId,
    Code,
    Color,
    Deaf,
    DefaultMessageNotifications,
    Deny,
    ExplicitContentFilter,
    Hoist,
    IconHash,
    Id,
    InviterId,
    MaxAge,
    MaxUses,
    Mentionable,
    MfaLevel,
    Mute,
    Name,
    Nick,
    Nsfw,
    OwnerId,
    PermissionOverwrites,
    Permissions,
    Position,
    PruneDeleteDays,
    #[serde(rename = "$add")]
    RoleAdded,
    #[serde(rename = "$remove")]
    RoleRemoved,
    Region,
    SplashHash,
    Temporary,
    Topic,
    Type,
    Uses,
    VanityUrlCode,
    VerificationLevel,
    WidgetChannelId,
    WidgetEnabled,
}