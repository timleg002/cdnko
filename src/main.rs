use actix_web::{
    get,
    post,
    web::{self, PayloadConfig},
    App,
    HttpResponse,
    HttpServer,
    Responder, Error, HttpRequest
};
use actix_multipart::Multipart;
use cdnko::files;
use cdnko::schema::{FileUploadRequest, ErrorResponse, FileResponse};
use cdnko::error::CdnkoError;
use mime_guess::mime;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/upload")]
async fn upload(
    payload: Multipart,
    request: HttpRequest
) -> Result<HttpResponse, Error> {
    let file_names = files
        ::save_file(payload)
        .await;

    match file_names {
        Ok(file_names) => Ok(
                HttpResponse
                    ::Ok()
                    .content_type("application/json")
                    .json(FileResponse {
                        file_names
                    })
        ),
        // TODO: handle the error
        Err(_error) => Err(CdnkoError::InternalError.into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(upload)
            .app_data(PayloadConfig::new(10_000_000_000)) // 10 GB
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}