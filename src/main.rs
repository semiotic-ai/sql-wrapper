use std::env;

use clickhouse_rs::Pool;
use execution::{GenericResult, DatabaseExecutor};
use poem::{listener::TcpListener, Route, Server, EndpointExt, middleware::Cors, web::Data};
use poem_openapi::{param::Query, payload::Json, OpenApi, OpenApiService, types::Any};

mod execution;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/execute", method = "get")]
    async fn index(&self, executor: Data<&DatabaseExecutor>, query: Query<String>) -> poem::Result<Json<Any<GenericResult>>> {
        println!("query: {:?}", query.0);
        
        let result = executor.execute_query(query.0).await?;
        Ok(Json(Any(result)))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let clickhouse_url =
        env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL environment must be set");

    let pool = Pool::new(clickhouse_url);
    let executor = DatabaseExecutor::new(pool);


    let api=env::var("EXPOSED_URL").unwrap_or("http://localhost:3000/api".to_string());

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server(api);
    let ui = api_service.swagger_ui();

    let router = Route::new().nest("/api", api_service).nest("/", ui)
        .with(Cors::new())
        .data(executor);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(router)
        .await
}
