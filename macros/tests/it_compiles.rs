#![allow(unused_must_use, dead_code)]
#![feature(async_fn_in_trait)]

use macros::service;

#[service]
mod foobar {

    pub struct Message {
        inner: String,
    }

    trait Foobar {
        #[get]
        async fn print_message(&self, message: Message) -> Message;
    }
}

// pub trait Foobar {
//     fn print_message(&self) -> Pin<Box<dyn Future<Output = String>>>;
// }

// pub trait FoobarServer {
//     fn print_message(&self) -> Pin<Box<dyn Future<Output = String>>>;
// }

// impl FoobarServer for Foobar {
//     fn print_message(&self) -> Pin<Box<dyn Future<Output = String>>> {
//         Box::pin(async move { String::from("Foo") })
//     }
// }
