use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(View::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(View::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(View::Title).string().not_null())
                    .col(ColumnDef::new(View::Text).string().not_null())
                    .col(ColumnDef::new(View::Reference).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(View::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum View {
    Table,
    Id,
    Title,
    Text,
    Reference,
}
