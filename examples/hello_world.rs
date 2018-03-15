#[macro_use]
extern crate tower_web;

#[macro_use]
extern crate serde_derive;

use tower_web::*;

#[derive(Clone, Debug)]
pub struct HelloWorld;

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    msg: &'static str,
}

impl_web! {
    impl HelloWorld {
        #[GET "/"]
        fn hello_world(&mut self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}