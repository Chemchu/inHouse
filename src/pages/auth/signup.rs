use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::localization::translator;

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate {
    title: &'static str,
    email: &'static str,
    password: &'static str,
    retry_password: &'static str,
    signup: &'static str,
    already_account: &'static str,
    login_here: &'static str,
    translator: fn(foo: &str) -> String,
}

pub async fn signup_page_handler() -> impl IntoResponse {
    let template = SignupTemplate {
        title: "Registrarse",
        email: "Correo electrónico",
        password: "Contraseña",
        retry_password: "Repetir contraseña",
        signup: "Registrarse",
        already_account: "¿Ya tienes cuenta?",
        login_here: "inicia sesión aquí",
        translator: todo!(),
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
