use sea_orm_migration::prelude::*;

use crate::m20231226_183754_create_table_vivienda::Viviendas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Productos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Productos::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Productos::Nombre).string().not_null())
                    .col(ColumnDef::new(Productos::Categoria).string().not_null())
                    .to_owned(),
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to create table {}", Productos::Table));

        manager
            .create_table(
                Table::create()
                    .table(ViviendaProducto::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ViviendaProducto::Id).uuid().not_null())
                    .col(
                        ColumnDef::new(ViviendaProducto::ProductoId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ViviendaProducto::Cantidad)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ViviendaProducto::Id)
                            .col(ViviendaProducto::ProductoId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vivienda_producto_to_vivienda")
                            .from(ViviendaProducto::Table, ViviendaProducto::Id)
                            .to(Viviendas::Table, Viviendas::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vivienda_producto_to_producto")
                            .from(ViviendaProducto::Table, ViviendaProducto::ProductoId)
                            .to(Productos::Table, Productos::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ViviendaProducto::Table).to_owned())
            .await
            .unwrap_or_else(|_| panic!("Failed to drop table {}", ViviendaProducto::Table));

        manager
            .drop_table(Table::drop().table(Productos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ViviendaProducto {
    Table,
    Id,
    ProductoId,
    Cantidad,
}

#[derive(DeriveIden)]
enum Productos {
    Table,
    Id,
    Nombre,
    Categoria,
}

impl std::fmt::Display for ViviendaProducto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViviendaProducto::Table => write!(f, "vivienda_producto"),
            ViviendaProducto::Id => write!(f, "vivienda_id"),
            ViviendaProducto::ProductoId => write!(f, "producto_id"),
            ViviendaProducto::Cantidad => write!(f, "cantidad"),
        }
    }
}

impl std::fmt::Display for Productos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Productos::Table => write!(f, "productos"),
            Productos::Id => write!(f, "id"),
            Productos::Nombre => write!(f, "nombre"),
            Productos::Categoria => write!(f, "categoria"),
        }
    }
}
