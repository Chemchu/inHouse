use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    title: &'static str,
    login: &'static str,
    email: &'static str,
    password: &'static str,
    no_account: &'static str,
    create_account: &'static str,
}

pub async fn login_page_handler() -> impl IntoResponse {
    let template = LoginTemplate {
        title: "Iniciar sesión",
        login: "Iniciar sesión",
        email: "Correo electrónico",
        password: "Contraseña",
        no_account: "¿No tienes cuenta?",
        create_account: "Crear cuenta",
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
