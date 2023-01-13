use clap::Parser;
use std::{sync::Mutex, time::Duration};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[derive(Parser, Debug)]
#[command(author, version, about = "Starts a api server, which will response with the given delay.\n\nA request to http://localhost:3000/1000 will response after 1 second.", long_about = None)]
struct Args {
    #[arg(default_value_t = 3000)]
    port: u16
}

struct AppState {
    count: Mutex<u64>,
}

#[get("/{duration}")]
async fn wait(path: web::Path<u64>, data: web::Data<AppState>) -> impl Responder {
    let ms = path.into_inner();

    {
        let mut call_count = data.count.lock().unwrap();
        *call_count += 1;
        println!("#{}: wait for {}ms ...", call_count, ms);
    }

    actix_web::rt::time::sleep(Duration::from_millis(ms)).await;
    HttpResponse::Ok().body(format!("waited {ms} milliseconds."))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let counter = web::Data::new(AppState {
        count: Mutex::new(0),
    });
    println!("Listening on port {} ...", args.port);
    HttpServer::new(move || App::new().app_data(counter.clone()).service(wait))
        .bind(("127.0.0.1", args.port))?
        .run()
        .await
        .map(|_| println!("server is running on 9090"))
}
