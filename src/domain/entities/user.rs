use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users", schema_name = "auth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub instance_id: Uuid,
    pub aud: Option<String>,
    pub role: Option<String>,
    #[sea_orm(unique)]
    pub email: String,
    pub email_confirmed_at: DateTime<FixedOffset>,
    pub invited_at: Option<DateTime<FixedOffset>>,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<DateTime<FixedOffset>>,
    pub recovery_token: Option<String>,
    pub recovery_sent_at: Option<DateTime<FixedOffset>>,
    pub email_change_token_new: Option<String>,
    pub email_change: Option<String>,
    pub email_change_sent_at: Option<String>,
    pub last_sign_in_at: Option<DateTime<FixedOffset>>,
    pub raw_app_meta_data: Json,
    pub raw_user_meta_data: Json,
    pub is_super_admin: Option<bool>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub phone: Option<String>,
    pub phone_confirmed_at: Option<DateTime<FixedOffset>>,
    pub phone_change: Option<String>,
    pub phone_change_token: Option<String>,
    pub phone_change_sent_at: Option<DateTime<FixedOffset>>,
    pub confirmed_at: Option<DateTime<FixedOffset>>,
    pub email_change_token_current: Option<String>,
    pub email_change_confirm_status: i16,
    pub banned_until: Option<DateTime<FixedOffset>>,
    pub reauthentication_token: Option<String>,
    pub reauthentication_sent_at: Option<DateTime<FixedOffset>>,
    pub is_sso_user: bool,
    pub deleted_at: Option<DateTime<FixedOffset>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
