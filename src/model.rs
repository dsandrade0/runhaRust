use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

//noinspection RsExternalLinter
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "pessoa")] // singular 'user' is a keyword..
pub struct Pessoa {
    pub id: Option<String>,
    pub nome: String,
    pub cpfCnpj: String,
    pub nascimento: Option<String>,
    pub seguros: Option<Vec<String>>
}

