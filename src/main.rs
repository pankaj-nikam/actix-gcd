use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Server is running on port 3000");
    server
        .bind("127.0.0.1:3000")
        .expect("Error binding server to address.")
        .run()
        .expect("Error running server");
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("n and m must be non-zero");
    } else {
        let mut x = form.n;
        let mut y = form.m;
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        let response = format!("GCD of {} and {} is <b>{}</b>", form.n, form.m, x);
        return HttpResponse::Ok().content_type("text/html").body(response);
    }
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <html>
                <head>
                    <title>GCD Calculator</title>
                </head>
                <body>
                    <h1>GCD Calculator</h1>
                    <form action="/gcd" method="post">
                        <input type="text" name="n"/>
                        <input type="text" name="m"/>
                        <button type="submit">Compute GCD</button>
                </body>
            </html>"#,
    )
}
