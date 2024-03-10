use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

struct Migrator {
    db: Surreal<Client>,
    migrations: Vec<&'static str>,
}

impl Migrator {
    pub async fn new() -> Self {
        Migrator {
            db: Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("Failed to connect to the server"),
            migrations: Vec::new(),
        }
    }

    pub fn add_migration(&mut self, migration: &'static str) -> &mut Self {
        self.migrations.push(migration);
        self
    }
}

trait MigratorTrait {
    fn migrations(&mut self);
    async fn up(&self) -> surrealdb::Result<()>;
    async fn down(&self) -> surrealdb::Result<()>;
}

impl MigratorTrait for Migrator {
    fn migrations(&mut self) {
        self.add_migration(include_str!("../database.surql"))
            .add_migration(include_str!("../database.surql"));
    }

    // TODO: Probar a ver como ejecutar todas las migraciones usando async/await o tokio para que
    // funcione db.query(...)
    async fn up(&self) -> Result<(), surrealdb::Error> {
        todo!()
    }

    async fn down(&self) -> surrealdb::Result<()> {
        todo!()
    }
}
