use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_vivienda", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub vivienda_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_metadata::Entity",
        from = "Column::UserId",
        to = "super::user_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserMetadata,

    #[sea_orm(
        belongs_to = "super::viviendas::Entity",
        from = "Column::ViviendaId",
        to = "super::viviendas::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Viviendas,
}

impl Related<super::user_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMetadata.def()
    }
}

impl Related<super::viviendas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Viviendas.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
