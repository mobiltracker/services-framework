#![feature(prelude_import)]
#![allow(unused_must_use, dead_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use macros::service;
use std::{future::Future, pin::Pin};
pub struct Message {
    inner: String,
}
pub trait Foobar {
    fn print_message(&self, message: Message) -> Box<Message>;
}
