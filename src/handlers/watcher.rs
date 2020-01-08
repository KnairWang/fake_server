use actix_web::{delete, get, patch, post, put, web, HttpRequest, HttpResponse, Responder, HttpMessage};

pub fn handler_config(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_all_files);
    cfg.route("*", web::to(all));
}

#[get("*")]
async fn get_all_files(req: HttpRequest) -> impl Responder {
    println!("{}", req.path());
    // println!("{:?}", req.payload);
    println!("{}", req.method());
    HttpResponse::Ok().body("hello")
}

async fn all(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("hello")
}
