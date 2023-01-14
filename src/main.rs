use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use std::{sync::Mutex, time::Duration};
use rlimit::Resource;
use thousands::Separable;

#[derive(Parser, Debug)]
#[command(author, version, about = "Starts an api server, which will response to requests with a given delay.\n\nIn example, a GET request to http://localhost:3000/1000 will response after 1 second.", long_about = None)]
struct Args {
    #[arg(default_value_t = 3000)]
    port: u16,
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

fn open_file_limit() -> Result<u64,String> {
    let (limit, _) =
        rlimit::getrlimit(Resource::NOFILE).map_err(|e| e.to_string())?;
    Ok(limit)
}

fn print_limit_if_available(limit: Result<u64, String>) {
    match limit {
        Ok(limit) =>
            if limit < 256 {
                println!("\x1b[93mNote\x1b[0m: You file limit is set to {}. This will limit the maximum amount of concurrent current connections.", limit.separate_with_commas());
            } else {
                println!("\x1b[93mNote\x1b[0m: You file limit is set to {}.", limit.separate_with_commas());
            },
        Err(_) => 
            println!("\x1b[93mNote\x1b[0m: Unable to check file limit. This might limit the possible concurrent connections."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let open_file_limit = open_file_limit();

    let counter = web::Data::new(AppState {
        count: Mutex::new(0),
    });

    println!("Listening on port {} ...", args.port);
    print_limit_if_available(open_file_limit);

    HttpServer::new(move || App::new().app_data(counter.clone()).service(wait))
        .bind(("127.0.0.1", args.port))?
        .run()
        .await
        .map(|_| println!("server is running on 9090"))
}
