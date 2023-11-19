use askama::Template;

#[derive(Template)]
#[template(path = "auth/signup/signup_success.html")]
pub struct SignUpSuccessMessage {
    pub translator: i18n::Translator,
}

#[derive(Template)]
#[template(path = "auth/signup/signup_fail.html")]
pub struct SignUpFailMessage {
    pub translator: i18n::Translator,
}

#[derive(Template)]
#[template(path = "auth/signup/signup_server_error.html")]
pub struct SignUpServerErrorMessage {
    pub translator: i18n::Translator,
}
