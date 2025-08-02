use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Email<T>
where
    T: Template + Send + Serialize + Default,
{
    pub to: String,
    pub from: String,
    pub subject: String,
    pub reply_to: Option<String>,
    pub template: T,
}

impl<T: Default> Default for Email<T>
where
    T: Template + Send + Serialize + Default,
{
    fn default() -> Self {
        Self {
            to: String::new(),
            from: String::from("admin@eckko.app"),
            subject: String::new(),
            reply_to: None,
            template: T::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailBuilder<T>
where
    T: Template + Send + Serialize + Default,
{
    to: String,
    from: String,
    subject: String,
    reply_to: Option<String>,
    template: T,
}

impl<T> EmailBuilder<T>
where
    T: Template + Send + Serialize + Default,
{
    pub fn new() -> Email<T> {
        Email::default()
    }

    pub fn to(mut self, to: &str) -> Self {
        if to.is_empty() {
            panic!("Email 'to' field cannot be empty");
        }
        self.to = to.to_string();
        self
    }

    pub fn from(mut self, from: &str) -> Self {
        self.from = from.to_string();
        self
    }

    pub fn subject(mut self, subject: &str) -> Self {
        if subject.is_empty() {
            panic!("Email 'subject' field cannot be empty");
        }
        self.subject = subject.to_string();
        self
    }
    pub fn reply_to(mut self, reply_to: &str) -> Self {
        self.reply_to = Some(reply_to.to_string());
        self
    }
}

impl<T> Email<T>
where
    T: Template + Send + Serialize + Default,
{
    pub fn build(self) -> EmailBuilder<T> {
        EmailBuilder {
            to: self.to,
            from: self.from,
            subject: self.subject,
            reply_to: self.reply_to,
            template: self.template,
        }
    }
}
