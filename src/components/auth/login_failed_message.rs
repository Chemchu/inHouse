use askama::Template;

#[derive(Template)]
#[template(path = "auth/login/login_failed_message.html")]
pub struct LoginFailedMessageTemplate {
    pub translator: crate::localization::Translator,
}
