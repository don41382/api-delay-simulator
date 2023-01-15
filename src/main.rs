use std::{sync::Mutex, time::Duration};

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use clap::{arg, ColorChoice, command, value_parser};
use thousands::Separable;

fn example_msg(port: u16) -> String {
    format!("Example: curl http://localhost:{port}/1000, will response after 1 second of delay.").to_string()
}

const ABOUT_MSG: &str = "Starts an api server, which will simulate latency to requests with a given delay.";

struct AppState {
    http_port: u16,
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
            HttpResponse::BadRequest().body(format!("please provide a valid duration, {}", example_msg(data.http_port)))
        }
    }
}

#[get("/")]
async fn help_service(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", example_msg(data.http_port)))
}

#[cfg(target_os = "windows")]
fn open_file_limit() -> Result<u64, String> {
    Err("not supported for windows".to_string())
}

#[cfg(not(target_os = "windows"))]
fn open_file_limit() -> Result<u64, String> {
    use rlimit::Resource;
    let (limit, _) =
        rlimit::getrlimit(Resource::NOFILE).map_err(|e| e.to_string())?;
    Ok(limit)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arguments = command!()
        .color(ColorChoice::Auto)
        .about(format!("{ABOUT_MSG}\n\n{}", example_msg(3000)))
        .arg(arg!([port] "server port").required(false).default_value("3000").value_parser(value_parser!(u16)))
        .get_matches();

    let http_port = arguments.get_one::<u16>("port").expect("port is required");

    let open_file_limit = open_file_limit();
    let request_counter = web::Data::new(AppState {
        http_port: *http_port,
        count: Mutex::new(0),
    });

    println!("Listening on port {}, file limit {} ...", *http_port, open_file_limit.map_or("unknown".to_string(), |x| x.separate_with_commas()));

    HttpServer::new(move ||
        App::new()
            .app_data(request_counter.clone())
            .service(wait_service)
            .service(help_service)
    )
        .bind(("127.0.0.1", *http_port))?
        .run()
        .await
}
