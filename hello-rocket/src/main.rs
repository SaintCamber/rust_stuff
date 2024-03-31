#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello,world!"
}
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/",rank=9)]
fn catch() -> &'static str {
    "requested page not found!"
}

#[launch]
fn rocket() -> _ {
    let app = rocket::build();
    app.mount("/",routes![index,hello,catch])
}