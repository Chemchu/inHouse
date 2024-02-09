struct Migrator;

trait MigratorTrait {
    async fn migrations() -> surrealdb::Result<()>;
}

impl MigratorTrait for Migrator {
    async fn migrations() -> surrealdb::Result<()> {
        // Add migrations here
        todo!()
    }
}
