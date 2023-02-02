use std::io::Result;

use actix_web::{App, HttpServer, Responder, HttpResponse, route};

#[actix_web::main]
async fn main() -> Result<()> {
    let app = move || {
        App::new()
            .service(graphql)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}

#[route("/graphql", method = "POST")]
async fn graphql() -> impl Responder {
    HttpResponse::Ok()
}

