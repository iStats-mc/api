use std::{fs, io, time::Duration};

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use api::{ApiResponse, Player};
use dotenv_codegen::dotenv;
use moka::future::Cache;
use regex::bytes::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, Client, ClientBuilder,
};
use serde_json::json;
use tokio::time::Instant;
use tracing::info;

struct IStatsApiState {
    pub by_name_query: String,
    pub by_uuid_query: String,
    pub client: Client,
    pub cache: Cache<String, Player>,
}

#[get("/player/{id}")]
async fn get_player(path: web::Path<String>, data: Data<IStatsApiState>) -> impl Responder {
    let start = Instant::now();
    let id = path.into_inner();

    let cache_result = data.cache.get(&id).await;
    if let Some(cache_result) = cache_result {
        info!("cached request served within {:.2?}", start.elapsed());
        return HttpResponse::Ok().json(cache_result);
    }

    let uuid_regex =
        Regex::new(r"\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b")
            .unwrap();
    let is_uuid = uuid_regex.is_match(id.as_bytes());
    let query = if is_uuid {
        &data.by_uuid_query
    } else {
        &data.by_name_query
    };
    let param = if is_uuid { "uuid" } else { "name" };

    let body = json!({
        "query": query,
        "variables": {
            param: id
        }
    })
    .to_string();

    let response = data
        .client
        .post("https://api.mccisland.net/graphql")
        .body(Body::from(body))
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();
    let parsed = serde_json::from_str::<ApiResponse>(text.as_str()).unwrap();

    if parsed.data.is_none() {
        return HttpResponse::BadRequest().await.unwrap();
    }

    let response_data = parsed.data.unwrap();
    if response_data.player.is_none() && response_data.player_by_username.is_none() {
        return HttpResponse::BadRequest().await.unwrap();
    }

    let player = if is_uuid {
        response_data.player.unwrap()
    } else {
        response_data.player_by_username.unwrap()
    };

    data.cache.insert(id, player.clone()).await;

    info!("uncached request served within {:.2?}", start.elapsed());
    HttpResponse::Ok().json(player)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let env = env_logger::Env::default()
        .filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);
    
    let api_key = dotenv!("MCCI_API_KEY").to_string();
    let by_name_query = fs::read_to_string("./schemas/player_by_name.graphql")?;
    let by_uuid_query = fs::read_to_string("./schemas/player_by_uuid.graphql")?;
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        "X-API-Key",
        HeaderValue::from_str(&api_key.as_str()).unwrap(),
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str("iStats/1.0 <radsteve@radsteve.net>").unwrap(),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let cache: Cache<String, Player> = Cache::builder()
        .name("iStats Players")
        .time_to_live(Duration::from_secs(5 * 60)) // 5 minute lifetime
        .max_capacity(64 * 1024 * 1024) // 64 MiB of data
        .build();

    let state = Data::new(IStatsApiState {
        by_name_query,
        by_uuid_query,
        client,
        cache,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .service(get_player)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
