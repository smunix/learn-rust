use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                <form/>
            "#,
    )
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                <form/>
            "#,
    )
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| App::new()
                                 .service(index)
                                 .route("/", web::get().to(get_index)));

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await;
}
