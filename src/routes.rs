use actix_web::{
    delete, get, post, put, web,
    error, guard, middleware, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};

use actix_files as fs;

#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("templates/favicon.ico")?)
}

// short link if string > 30 print WARN
#[post("/short")]
async fn shorten(todo: web::Json<TodoRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::create(todo.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Error trying to create new todo")
    }
}

// beautify 添加任意 /project/name/id slash

// redirect

// search

// list

// homepage

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(favicon);
}