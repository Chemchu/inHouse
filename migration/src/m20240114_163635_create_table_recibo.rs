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
                    .table(Recibos::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Recibos::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Recibos::Descripcion).string().not_null())
                    .col(ColumnDef::new(Recibos::Total).float().not_null())
                    .col(ColumnDef::new(Recibos::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to create table {}", Recibos::Table));

        manager
            .create_table(
                Table::create()
                    .table(UserRecibo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRecibo::Id).uuid().not_null())
                    .col(ColumnDef::new(UserRecibo::ReciboId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserRecibo::Id)
                            .col(UserRecibo::ReciboId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_recibo_to_user")
                            .from(UserRecibo::Table, UserRecibo::Id)
                            .to(UserMetadata::Table, UserMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_recibo_to_recibo")
                            .from(UserRecibo::Table, UserRecibo::ReciboId)
                            .to(Recibos::Table, Recibos::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRecibo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Recibos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRecibo {
    Table,
    Id,
    ReciboId,
}

#[derive(DeriveIden)]
enum Recibos {
    Table,
    Id,
    Descripcion,
    Total,
    CreatedAt,
}

impl std::fmt::Display for UserRecibo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRecibo::Table => write!(f, "user_recibo"),
            UserRecibo::Id => write!(f, "user_id"),
            UserRecibo::ReciboId => write!(f, "recibo_id"),
        }
    }
}

impl std::fmt::Display for Recibos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Recibos::Table => write!(f, "recibos"),
            Recibos::Id => write!(f, "id"),
            Recibos::Descripcion => write!(f, "descripcion"),
            Recibos::Total => write!(f, "total"),
            Recibos::CreatedAt => write!(f, "created_at"),
        }
    }
}
