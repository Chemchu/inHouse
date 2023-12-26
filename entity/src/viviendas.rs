use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "viviendas", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub street: String,
    pub street_number: String,
    pub apartment_number: Option<String>,
    pub place_type: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub additional_info: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::viviendas::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_vivienda::Relation::Viviendas.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_vivienda::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
