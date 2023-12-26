use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "compras", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub receipt_number: Option<String>,
    pub total: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::productos::Entity",
        from = "Column::Id",
        to = "super::productos::Column::Id"
    )]
    Users,
}

impl Related<super::productos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Productos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
