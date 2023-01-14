use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use std::{sync::Mutex, time::Duration};
use thousands::Separable;

#[derive(Parser, Debug)]
#[command(author, version, about = "Starts an api server, which will response to requests with a given delay.\n\nIn example, a GET request to http://localhost:3000/1000 will response after 1 second.", long_about = None)]
struct Args {
    #[arg(default_value_t = 3000)]
    port: u16,
}

struct AppState {
    port: u16,
    count: Mutex<u64>,
}

#[get("/{duration}")]
async fn wait_service(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let parsed_path =
        path.into_inner().parse::<u64>();

    match parsed_path {
        Ok(wait_duration) => {
            {
                let mut call_count = data.count.lock().unwrap();
                *call_count += 1;
                println!("#{}: wait for {}ms ...", call_count, wait_duration);
            }

            actix_web::rt::time::sleep(Duration::from_millis(wait_duration)).await;
            HttpResponse::Ok().body(format!("waited {wait_duration} milliseconds."))
        }
        Err(_) => {
            HttpResponse::BadRequest().body(format!("please provide a valid duration, e.g. http://localhost:{}/1000 to delay the response by a second", data.port))
        },
    }
}

#[get("/")]
async fn help_service(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Please request with a delay in millis in the path, e.g. http://localhost:{}/1000 to delay the response by a second.", data.port))
}

#[cfg(target_os = "windows")]
fn open_file_limit() -> Result<u64,String> {
    Err("not supported for windows".to_string())
}

#[cfg(not(target_os = "windows"))]
fn open_file_limit() -> Result<u64,String> {
    use rlimit::Resource;
    let (limit, _) =
        rlimit::getrlimit(Resource::NOFILE).map_err(|e| e.to_string())?;
    Ok(limit)
}

fn print_limit_if_available(limit: Result<u64, String>) {
    match limit {
        Ok(limit) =>
            println!("\x1b[93mNote\x1b[0m: You file limit is set to {}. This will limit the maximum amount of concurrent current connections.", limit.separate_with_commas()),
        Err(_) =>
            println!("Note: Unable to check file limit. This might limit the possible concurrent connections."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let open_file_limit = open_file_limit();

    let counter = web::Data::new(AppState {
        port: args.port,
        count: Mutex::new(0),
    });

    println!("Listening on port {} ...", args.port);
    print_limit_if_available(open_file_limit);

    HttpServer::new(move ||
            App::new()
                .app_data(counter.clone())
                .service(wait_service)
                .service(help_service)
         )
        .bind(("127.0.0.1", args.port))?
        .run()
        .await
}
