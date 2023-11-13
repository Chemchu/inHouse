use askama::Template;

#[derive(Template)]
#[template(path = "auth/signup/signup_success.html")]
pub struct SignUpSuccessMessage {
    pub translator: crate::util::localization::Translator,
}

#[derive(Template)]
#[template(path = "auth/signup/signup_fail.html")]
pub struct SignUpFailMessage {
    pub translator: crate::util::localization::Translator,
}

#[derive(Template)]
#[template(path = "auth/signup/signup_server_error.html")]
pub struct SignUpServerErrorMessage {
    pub translator: crate::util::localization::Translator,
}
