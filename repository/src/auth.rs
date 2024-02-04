use entity::usuario::Usuario;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn exists_by_email(db: &Surreal<Client>, email: &str) -> Result<bool, String> {
    let sql = "SELECT * FROM type::table($table) WHERE email = $email";

    let query_result = db
        .query(sql)
        .bind(("table", "user"))
        .bind(("email", email))
        .await;

    let user: Result<Vec<Usuario>, surrealdb::Error> = query_result
        .expect("SurrealQL should have queried User table")
        .take(0);

    Ok(!user
        .expect("SurrealQL take() should have retrieved the user")
        .is_empty())
}
