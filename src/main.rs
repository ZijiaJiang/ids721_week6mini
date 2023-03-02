// this is a web microservice that uses the actix-web framework
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use web::Path;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "This is a calculator microservice.\nUse the following format to make calculation\n
    /{add/subtract/multiply/divide}/{first_number}/{second_number}\n",
    )
}

#[get("/{operation}/{first_number}/{second_number}")]
async fn calculate(path: web::Path<(String, i32, i32)>) -> impl Responder {
    let (operation, first_number, second_number) = path.into_inner();
    let result = match operation.as_str() {
        "add" => first_number + second_number,
        "subtract" => first_number - second_number,
        "multiply" => first_number * second_number,
        "divide" => first_number / second_number,
        _ => 0,
    };
    HttpResponse::Ok().body(format!(
        "{} {} {} = {}",
        first_number, operation, second_number, result
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(calculate))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
