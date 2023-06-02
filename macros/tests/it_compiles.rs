#![allow(unused_must_use, dead_code)]
#![feature(async_fn_in_trait)]

use axum::{routing::get, Extension, Json, Router};
use macros::service;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct FoobarServer {
    mssql_conn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    inner: String,
}

#[service]
mod foobar {
    trait Foobar: Sized {
        #[get]
        async fn print_message(svc: Extension<Self>, message: Json<Message>) -> String;
    }
}

impl Foobar for FoobarServer {
    async fn print_message(svc: Extension<Self>, message: Json<Message>) -> String {
        todo!()
    }
}

impl FoobarServer {
    async fn serve(&self) {
        let app = Router::new().route("print_message", get(FoobarServer::print_message));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
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
