use sqlx::postgres::{PgArguments, PgPool, PgPoolOptions};
use sqlx::Arguments;
use std::env;

pub struct DbPool {
    pool: PgPool,
}

#[derive(Debug)]
pub enum InputValue {
    Integer(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
    // Add more types as needed
}

#[derive(Debug)]
pub enum ParamType {
    In(InputValue),
    InOut(String),
    Out(String),
}

impl DbPool {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Ok(DbPool { pool })
    }

    pub async fn execute_stored_proc(
        &self,
        proc_name: &str,
        params: Vec<ParamType>,
    ) -> Result<
        (Vec<sqlx::postgres::PgRow>, Vec<sqlx::postgres::PgRow>),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let mut query = format!("CALL {}(", proc_name);
        let mut args = PgArguments::default();
        let mut out_params = Vec::new();

        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                query.push_str(", ");
            }
            match param {
                ParamType::In(value) => {
                    query.push_str(&format!("${}", i + 1));
                    match value {
                        InputValue::Integer(x) => args.add(x)?,
                        InputValue::Float(x) => args.add(x)?,
                        InputValue::Text(x) => args.add(x)?,
                        InputValue::Boolean(x) => args.add(x)?,
                        // Add more cases as needed
                    }
                }
                ParamType::InOut(type_name) | ParamType::Out(type_name) => {
                    let param_name = format!("p{}", i + 1);
                    query.push_str(&format!("INOUT {} {}", param_name, type_name));
                    out_params.push(param_name);
                }
            }
        }

        query.push_str(")");

        let result = sqlx::query_with(&query, args).fetch_all(&self.pool).await?;

        let outputs = if !out_params.is_empty() {
            let out_query = format!("SELECT {} FROM pg_temp", out_params.join(", "));
            sqlx::query(&out_query).fetch_all(&self.pool).await?
        } else {
            Vec::new()
        };

        Ok((result, outputs))
    }
}
