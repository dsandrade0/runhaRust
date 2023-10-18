use actix_web::{get, http, post, Responder, web};
use deadpool_postgres::{GenericClient, Pool};
use serde::Deserialize;
use time::Date;
use time::macros::format_description;
use uuid::Uuid;

use crate::model::Pessoa;

#[post("/pessoa")]
async fn inserir_pessoa(pool: web::Data<Pool>, req: web::Json<Pessoa>) -> impl Responder {
    let my_uuid = Uuid::new_v4();
    let client = pool.get().await.unwrap();
    let nome = &req.nome;
    let cpf_cnpj = &req.cpfCnpj;
    let format = format_description!("[year]-[month]-[day]");

    let nascimento = Date::parse(req.nascimento.clone().unwrap().as_str(), format);

   return  match nascimento {
        Ok(_) => {
            let seguros = &req.seguros;

            let res = client.execute(
                "insert into pessoa (id, nome, cpfcnpj, nascimento, seguros) values ($1,$2,$3,$4,$5);",
                &[&my_uuid, &nome, &cpf_cnpj, &nascimento.unwrap(), &seguros])
                .await;

            print!(" - Tentando cadastrar");
            match res {
                Ok(_n) => (web::Json(
                    Some(Pessoa{
                        id: Some(my_uuid.to_string()),
                        nome: req.nome.to_string(),
                        cpfCnpj: req.cpfCnpj.to_string(),
                        nascimento: req.nascimento.to_owned(),
                        seguros: req.seguros.to_owned()
                    })), http::StatusCode::CREATED),
                Err(_e) => (web::Json(None), http::StatusCode::BAD_REQUEST),
            }
        }
        Err(_) => {
            return (web::Json(None), http::StatusCode::BAD_REQUEST)
        }
    }
}

#[derive(Deserialize)]
struct BuscaPessoa {
    id: Uuid
}

#[get("/pessoa/{id}")]
async fn buscar_pessoa(pool: web::Data<Pool>, id: web::Path<BuscaPessoa>) -> impl Responder {

    let client = pool.get().await.unwrap();
    let stm = client.prepare_cached(
        "SELECT * FROM pessoa where id = $1;").await.unwrap();
    let row = client.query(&stm, &[&id.id]).await.unwrap();


    if row.is_empty() {
        return (web::Json(None), http::StatusCode::OK);
    }

    println!("Buscar {}", id.id);

    let uuid: Uuid = row[0].get(0);
    let date: Date = row[0].get(3);
    let seg: Vec<String> = row[0].get(4);


    (web::Json(Some(Pessoa{
        id: Some(uuid.to_string()),
        nome: row[0].get(1),
        cpfCnpj: row[0].get(2),
        nascimento: Some(date.to_string()),
        seguros: Some(seg)
    })), http::StatusCode::OK)
}

#[derive(Deserialize)]
struct Busca {
    t: String
}

#[get("/pessoa")]
async fn busca_termo(pool: web::Data<Pool>, query: web::Query<Busca>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let stmt = client.prepare_cached(
        "select id,nome,cpfcnpj,nascimento,seguros from (select *, unnest(seguros) as tag from pessoa) x where x.tag ilike $1 or x.nome ilike $1 limit 50")
        .await.unwrap();

    let search_query = "%".to_owned() + &query.t.to_string() + "%";

    let rows = client.query(&stmt, &[&search_query])
        .await
        .unwrap();

    let mut res: Vec<Pessoa> = vec![];
    for row in rows {
        let uuid: Uuid = row.get(0);
        let date: Date = row.get(3);
        let seg: Vec<String> = row.get(4);
        res.push(

            Pessoa{
                id: Some(uuid.to_string()),
                nome: row.get(1),
                cpfCnpj: row.get(2),
                nascimento: Some(date.to_string()),
                seguros: Some(seg)
            }
        )
    }

    (web::Json(res), http::StatusCode::OK)
}
