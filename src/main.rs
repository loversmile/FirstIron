// fn main() {
//     println!("Hello, world!");
// }

extern crate iron;
extern crate router;
extern crate rustc_serialize;

//use std::io;
use std::io::Read;
//use std::fs::File;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use router::Router;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn new_func (_: &mut Request) -> IronResult<Response> {
    let greeting = Greeting {msg: "Hi mumu\n".to_string()};
    let payload = json::encode(&greeting).unwrap();
    //Ok(Response::with((status::Ok, "Hi RUST!\n")))
    Ok(Response::with((status::Ok, payload)))
}

fn set_g (request: &mut Request) -> IronResult<Response> {
    //let payload = request.body.read_to_string();
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    //let payload = json::encode(&greeting).unwrap();
    let request: Greeting = json::decode(&payload).unwrap();
    let greeting = Greeting {msg: request.msg};
    let payload = json::encode(&greeting).unwrap();
    //Ok(Response::with((status::Ok, "Hi RUST!\n")))
    let s = format!("Result-{}", payload);
    Ok(Response::with((status::Ok, s)))
}

fn main() {
    // Iron::new(|_: &mut Request| {
    //     Ok(Response::with((status::Ok, "Hi RUST!\n")))
    // }).http("0.0.0.0:3000").unwrap();
    //Iron::new(new_func).http("0.0.0.0:3000").unwrap();
    let mut router = Router::new();

    router.get("/", new_func, "");
    router.post("/set", set_g, "set");

    Iron::new(router).http("0.0.0.0:3333").unwrap();
}