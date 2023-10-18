use std::time::Duration;
use actix_web::{App, HttpServer, web};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::routes::{buscar_pessoa, busca_termo, inserir_pessoa};

mod routes;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.dbname = Some("rinha".to_string());
    cfg.host = Some("db".to_string());
    cfg.port = Some(5432);
    cfg.user = Some("rinha".to_string());
    cfg.password = Some("rinha".to_string());

    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let server = HttpServer::new(move|| {
        App::new().app_data(web::Data::new(pool.clone()))
            .service(busca_termo)
            .service(buscar_pessoa)
            .service(inserir_pessoa)
    })
        .bind(("0.0.0.0", 8080))?
        .client_request_timeout(Duration::from_secs(10))
        .client_disconnect_timeout(Duration::from_secs(10))
        .run();

    println!("Server running at http://{}/", "0.0.0.0:8080");

    server.await
}


