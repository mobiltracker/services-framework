#![allow(unused_must_use, dead_code)]

use std::{future::Future, pin::Pin};

use macros::service;
pub struct Message {
    inner: String,
}

#[service]
pub mod foobar {
    trait Foobar {
        async fn print_message(&self, message: Message) -> Message;
        async fn foobar_message(&self, message: Message) -> String;
    }
}

// pub trait Foobar {
//     fn print_message(&self, message: Message) -> Box<Message>;
// }

// pub trait FoobarServer {
//     fn print_message(&self) -> Pin<Box<dyn Future<Output = String>>>;
// }

// impl FoobarServer for Foobar {
//     fn print_message(&self) -> Pin<Box<dyn Future<Output = String>>> {
//         Box::pin(async move { String::from("Foo") })
//     }
// }
