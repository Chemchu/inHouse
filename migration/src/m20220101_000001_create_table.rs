use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let table_creation_result = manager
            .create_table(
                Table::create()
                    .table(UserMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserMetadata::Id)
                            .uuid()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserMetadata::Username).string().unique_key())
                    .col(ColumnDef::new(UserMetadata::Name).string())
                    .col(ColumnDef::new(UserMetadata::Surname).string())
                    .col(ColumnDef::new(UserMetadata::Birthdate).date())
                    .to_owned(),
            )
            .await;

        table_creation_result
            .unwrap_or_else(|_| panic!("Failed to create table {}", UserMetadata::Table));

        // This table is always created by Supabase and is not part of the migration.
        db.execute_unprepared(
            "alter table public.user_metadata add constraint fk_user_metadata_user_id 
            foreign key (id) references auth.users (id) on delete cascade;",
        )
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Failed to create foreign key for {} because of {}",
                UserMetadata::Table,
                e
            )
        });

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserMetadata::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserMetadata {
    Table,
    Id,
    Username,
    Name,
    Surname,
    Birthdate,
}

impl std::fmt::Display for UserMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserMetadata::Table => write!(f, "user_metadata"),
            UserMetadata::Id => write!(f, "id"),
            UserMetadata::Username => write!(f, "username"),
            UserMetadata::Name => write!(f, "name"),
            UserMetadata::Surname => write!(f, "surname"),
            UserMetadata::Birthdate => write!(f, "birthdate"),
        }
    }
}
