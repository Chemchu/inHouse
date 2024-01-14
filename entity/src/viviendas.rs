use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "viviendas", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub nombre: String,
    pub calle: String,
    pub numer_calle: String,
    pub numero_apartamento: Option<String>,
    pub tipo_vivienda: Option<String>,
    pub codigo_postal: String,
    pub ciudad: String,
    pub estado: String,
    pub pais: String,
    pub latitud: f64,
    pub longitud: f64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub informacion_adicional: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::user_vivienda::Entity",
        from = "Column::Id",
        to = "super::user_vivienda::Column::ViviendaId"
    )]
    UserVivienda,

    #[sea_orm(
        has_many = "super::vivienda_producto::Entity",
        from = "Column::Id",
        to = "super::vivienda_producto::Column::ViviendaId"
    )]
    ViviendaProducto,
}

// TODO: fix this Gus, these relationships are weird to understand
// impl Related<super::user_vivienda::Entity> for Entity {
//     fn to() -> RelationDef {
//         super::user_vivienda::Relation::UserVivienda.def()
//     }
// }
// impl Related<super::vivienda_producto::Entity> for Entity {
//     fn to() -> RelationDef {
//         super::vivienda_producto::Relation::Vivendas.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
