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
                    .table(Viviendas::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Viviendas::Id).uuid().primary_key())
                    .col(ColumnDef::new(Viviendas::Name).string().not_null())
                    .col(ColumnDef::new(Viviendas::Street).string().not_null())
                    .col(ColumnDef::new(Viviendas::StreetNumber).integer().not_null())
                    .col(ColumnDef::new(Viviendas::ApartmentNumber).integer())
                    .col(ColumnDef::new(Viviendas::PlaceType).string())
                    .col(ColumnDef::new(Viviendas::PostalCode).integer().not_null())
                    .col(ColumnDef::new(Viviendas::City).string().not_null())
                    .col(ColumnDef::new(Viviendas::State).string().not_null())
                    .col(ColumnDef::new(Viviendas::Country).string().not_null())
                    .col(ColumnDef::new(Viviendas::Latitude).float().not_null())
                    .col(ColumnDef::new(Viviendas::Longitude).float().not_null())
                    .col(ColumnDef::new(Viviendas::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Viviendas::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Viviendas::AdditionalInfo).string())
                    .to_owned(),
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to create table {}", Viviendas::Table));

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Viviendas::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Viviendas {
    Table,
    Id,
    Name,
    Street,
    StreetNumber,
    ApartmentNumber,
    PlaceType,
    PostalCode,
    City,
    State,
    Country,
    Latitude,
    Longitude,
    CreatedAt,
    UpdatedAt,
    AdditionalInfo,
}

impl std::fmt::Display for Viviendas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Viviendas::Table => write!(f, "user_metadata"),
            Viviendas::Id => write!(f, "id"),
            Viviendas::Name => write!(f, "name"),
            Viviendas::Street => write!(f, "street"),
            Viviendas::StreetNumber => write!(f, "street_number"),
            Viviendas::ApartmentNumber => write!(f, "apartment_number"),
            Viviendas::PlaceType => write!(f, "place_type"),
            Viviendas::PostalCode => write!(f, "postal_code"),
            Viviendas::City => write!(f, "city"),
            Viviendas::State => write!(f, "state"),
            Viviendas::Country => write!(f, "country"),
            Viviendas::Latitude => write!(f, "latitude"),
            Viviendas::Longitude => write!(f, "longitude"),
            Viviendas::CreatedAt => write!(f, "created_at"),
            Viviendas::UpdatedAt => write!(f, "updated_at"),
            Viviendas::AdditionalInfo => write!(f, "additional_info"),
        }
    }
}
