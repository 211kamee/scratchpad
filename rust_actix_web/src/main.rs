use actix_cors::Cors;
use actix_web::{web::Json, *};
use serde::*;
use std::{
    fs::{read_to_string, write},
    sync::Mutex,
};

#[derive(Deserialize)]
struct MsgRequest {
    user_prompt: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct LogEntry {
    token: String,
    user_prompt: String,
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u16,
}

#[derive(Serialize, Deserialize)]
struct Login {
    user: String,
    key: String,
}

#[derive(Serialize, Deserialize)]
struct CollectiveData {
    visits: u128,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut _port: u16 = 8080;
    let file = read_to_string(String::from("./assets/data.json")).unwrap_or_else(|err| {
        eprintln!("{}", err);
        println!("Attempting to create the file");
        write("./assets/data.json", String::new()).unwrap();
        return String::new();
    });
    let data = match serde_json::from_str(&file) {
        Ok(res) => res,
        Err(_e) => CollectiveData { visits: 0 },
    };

    let collective_data = web::Data::new(Mutex::new(data));

    let existing_logs: Vec<LogEntry> = read_to_string("./assets/temp-logs.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    let logs: web::Data<Mutex<Vec<LogEntry>>> = web::Data::new(Mutex::new(existing_logs));

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(collective_data.clone())
            .app_data(logs.clone())
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body(format!("API is live.")) }),
            )
            .service(
                web::scope("/api")
                    .service(msg)
                    .service(get_logs)
                    .service(home)
                    .service(display)
                    .service(displaymsg)
                    .service(query_param)
                    .service(login)
                    .service(visits),
            )
            .default_service(web::to(not_found))
    })
    .bind(("0.0.0.0", _port))?;

    println!("Bound to : http://localhost:{}", _port);

    server.run().await
}

#[post("/msg")]
async fn msg(
    req: HttpRequest,
    body: Json<MsgRequest>,
    logs: web::Data<Mutex<Vec<LogEntry>>>,
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("")
        .to_string();

    let entry = LogEntry {
        token,
        user_prompt: body.user_prompt.clone(),
    };

    let mut log_store = logs.lock().unwrap();
    log_store.push(entry);

    let _ = write(
        "./assets/temp-logs.json",
        serde_json::to_string_pretty(&*log_store).unwrap(),
    );

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Agent is not active. However, your req is stored"
    }))
}

#[get("/logs")]
async fn get_logs() -> impl Responder {
    let content = read_to_string("./assets/temp-logs.json").unwrap_or_else(|_| "[]".to_string());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(content)
}

#[get("/home")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("/home")
}

#[get("/display")]
async fn display() -> impl Responder {
    HttpResponse::Ok().body("/display")
}

#[get("/visits")]
async fn visits(collective_data: web::Data<Mutex<CollectiveData>>) -> impl Responder {
    let mut collection_data = collective_data.lock().unwrap();
    collection_data.visits += 1;

    let updated_data = serde_json::to_string_pretty(&(*collection_data)).unwrap();
    write("./assets/data.json", updated_data).unwrap();
    let response_msg = format!("/visits\n\nTotal Visits : {}", collection_data.visits);
    HttpResponse::Ok().body(response_msg)
}

#[get("/display/{msg}")]
async fn displaymsg(path: web::Path<String>) -> impl Responder {
    // let msg = format!("/display/{}", path.into_inner());
    HttpResponse::Ok().body(path.into_inner())
}

#[get("/query")]
async fn query_param(query: web::Query<Info>) -> impl Responder {
    let this_msg = format!("name: {},\nage: {}", query.name, query.age);
    HttpResponse::Ok().body(this_msg)
}

#[post("/login")]
async fn login(data: Json<Login>) -> impl Responder {
    // let msg = format!("{{user: {},key: {}}}", data.user, data.key);
    // serde_json::to_string(&datunwrap() // is responding with a string, not json
    HttpResponse::Ok().json(data)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Route not found!")
}
