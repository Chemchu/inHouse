use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table_user::UserMetadata;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Viviendas::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Viviendas::Id).uuid().primary_key())
                    .col(ColumnDef::new(Viviendas::Nombre).string().not_null())
                    .col(ColumnDef::new(Viviendas::Calle).string().not_null())
                    .col(ColumnDef::new(Viviendas::NumeroCalle).integer().not_null())
                    .col(ColumnDef::new(Viviendas::NumeroApartamento).integer())
                    .col(ColumnDef::new(Viviendas::TipoVivienda).string())
                    .col(ColumnDef::new(Viviendas::CodigoPostal).integer().not_null())
                    .col(ColumnDef::new(Viviendas::Ciudad).string().not_null())
                    .col(ColumnDef::new(Viviendas::Estado).string().not_null())
                    .col(ColumnDef::new(Viviendas::Pais).string().not_null())
                    .col(ColumnDef::new(Viviendas::Latitud).float().not_null())
                    .col(ColumnDef::new(Viviendas::Longitud).float().not_null())
                    .col(ColumnDef::new(Viviendas::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Viviendas::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Viviendas::InformacionAdicional).string())
                    .to_owned(),
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to create table {}", Viviendas::Table));

        manager
            .create_table(
                Table::create()
                    .table(UserVivienda::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserVivienda::Id).uuid().not_null())
                    .col(ColumnDef::new(UserVivienda::ViviendaId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserVivienda::Id)
                            .col(UserVivienda::ViviendaId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_vivienda_to_user")
                            .from(UserVivienda::Table, UserVivienda::Id)
                            .to(UserMetadata::Table, UserMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_vivienda_to_vivienda")
                            .from(UserVivienda::Table, UserVivienda::ViviendaId)
                            .to(Viviendas::Table, Viviendas::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to create table {}", UserVivienda::Table));

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Viviendas::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserVivienda::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Viviendas {
    Table,
    Id,
    Nombre,
    Calle,
    NumeroCalle,
    NumeroApartamento,
    TipoVivienda,
    CodigoPostal,
    Ciudad,
    Estado,
    Pais,
    Latitud,
    Longitud,
    CreatedAt,
    UpdatedAt,
    InformacionAdicional,
}

impl std::fmt::Display for Viviendas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Viviendas::Table => write!(f, "viviendas"),
            Viviendas::Id => write!(f, "id"),
            Viviendas::Nombre => write!(f, "nombre"),
            Viviendas::Calle => write!(f, "calle"),
            Viviendas::NumeroCalle => write!(f, "numero_calle"),
            Viviendas::NumeroApartamento => write!(f, "numero_apartamento"),
            Viviendas::TipoVivienda => write!(f, "tipo_vivienda"),
            Viviendas::CodigoPostal => write!(f, "codigo_postal"),
            Viviendas::Ciudad => write!(f, "ciudad"),
            Viviendas::Estado => write!(f, "estado"),
            Viviendas::Pais => write!(f, "pais"),
            Viviendas::Latitud => write!(f, "latitud"),
            Viviendas::Longitud => write!(f, "longitud"),
            Viviendas::CreatedAt => write!(f, "created_at"),
            Viviendas::UpdatedAt => write!(f, "updated_at"),
            Viviendas::InformacionAdicional => write!(f, "informacion_adicional"),
        }
    }
}

#[derive(DeriveIden)]
enum UserVivienda {
    Table,
    Id,
    ViviendaId,
}

impl std::fmt::Display for UserVivienda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserVivienda::Table => write!(f, "user_vivienda"),
            UserVivienda::Id => write!(f, "user_id"),
            UserVivienda::ViviendaId => write!(f, "vivienda_id"),
        }
    }
}
