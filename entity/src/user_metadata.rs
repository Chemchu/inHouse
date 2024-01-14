use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_metadata", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub birthdate: Option<DateTime<FixedOffset>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::Id",
        to = "super::users::Column::Id"
    )]
    Users,

    #[sea_orm(
        has_many = "super::user_vivienda::Entity",
        from = "Column::Id",
        to = "super::user_vivienda::Column::UserId"
    )]
    UserVivienda,

    #[sea_orm(
        has_many = "super::user_recibo::Entity",
        from = "Column::Id",
        to = "super::user_recibo::Column::UserId"
    )]
    UserRecibo,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::viviendas::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_vivienda::Relation::Viviendas.def()
    }
}

impl Related<super::recibos::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_recibo::Relation::Recibos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
