#[macro_use] extern crate rocket;
use rocket::Request;

#[get("/")]
fn index() -> &'static str {
    "hello,world!"
}
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .register("/",catchers![not_found])
    .mount("/",routes![index,hello])


}