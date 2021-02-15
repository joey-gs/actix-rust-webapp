use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use tera::{Context, Tera};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct User{
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser{
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Post{
    title: String,
    link: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");
    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn signup_process(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn login_process(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged in: {}", data.username))
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let posts = [
        Post{
            title: String::from("This is the first link"),
            link: String::from("https://example.com"),
            author: String::from("Bob")
        },
        Post{
            title: String::from("The second link"),
            link: String::from("https://example.com"),
            author: String::from("Alice")
        }
    ];
    data.insert("title", "Home");
    data.insert("posts", &posts);
    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a post");
    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn submission_process(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Post submitted: {}", data.title))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(signup_process))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(login_process))
            .route("/", web::get().to(index))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(submission_process))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}