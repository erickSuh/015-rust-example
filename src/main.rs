#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/teste")]
fn teste() -> &'static str {
    "Função de teste3"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, teste])
}