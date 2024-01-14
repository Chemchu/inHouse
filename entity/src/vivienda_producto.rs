use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "vivienda_producto", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub vivienda_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub producto_id: Uuid,
    pub cantidad: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::viviendas::Entity",
        from = "Column::ViviendaId",
        to = "super::viviendas::Column::Id"
    )]
    Vivendas,

    #[sea_orm(
        belongs_to = "super::productos::Entity",
        from = "Column::ProductoId",
        to = "super::productos::Column::Id"
    )]
    Productos,
}

impl Related<super::viviendas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Vivendas.def()
    }
}

impl Related<super::productos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Productos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
