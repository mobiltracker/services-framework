use macros::service;

pub struct Foobar {
    inner: String,
}

#[service]
pub mod foobar {
    use crate::Foobar;

    struct Message {
        inner: String,
    }

    impl Foobar {
        async fn print_message(&self, message: Message) -> Message {
            message
        }

        async fn foobar_message(&self, message: Message) -> String {
            message.inner
        }
    }
}

impl Foobar {}
fn main() {}
