use askama::Template;

#[derive(Template)]
#[template(path = "message_success.html")]
pub struct SignUpSuccessMessage {
    pub translator: crate::localization::Translator,
}

#[derive(Template)]
#[template(path = "message_fail.html")]
pub struct SignUpFailMessage {
    pub translator: crate::localization::Translator,
}
