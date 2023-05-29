#![feature(prelude_import)]
#![allow(unused_must_use, dead_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{future::Future, pin::Pin};
use macros::service;
pub struct Message {
    inner: String,
}
pub trait Foobar {
    fn print_message(&self, message: Message) -> Message;
}
#[rustc_main]
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
