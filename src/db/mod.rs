use crate::models::DbDecimal;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::*;
use serde_json::Value as JsonValue;
use thiserror::Error;

use std::collections::HashMap;
use std::env;

pub struct DbPool {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Error getting connection from pool: {0}")]
    PoolError(#[from] diesel::r2d2::PoolError),
    #[error("Error executing stored procedure: {0}")]
    ExecutionError(String),
}

pub enum SqlParam {
    // integer types
    Integer(Option<i32>),
    BigInt(Option<i64>),
    SmallInt(Option<i16>),
    Double(Option<DbDecimal>),

    // text types
    Text(Option<String>),
    Varchar(Option<String>),

    // other types
    Boolean(Option<bool>),
    Date(Option<chrono::NaiveDate>),
    Timestamp(Option<chrono::NaiveDateTime>),
    Uuid(Option<uuid::Uuid>),
    JsonB(Option<serde_json::Value>),

    // array types
    IntArray(Option<Vec<i32>>),
    TextArray(Option<Vec<String>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParamDirection {
    In,
    Out,
}

#[derive(QueryableByName, Debug)]
struct JsonResults {
    #[diesel(sql_type = Json)]
    json: JsonValue,
}

#[derive(QueryableByName, Debug)]
#[diesel(check_for_backend(Pg))]
pub struct OutParam {
    #[diesel(sql_type = Nullable<Text>)]
    pub out_param: Option<String>,
}

impl SqlParam {
    fn get_type_name(&self) -> &'static str {
        match self {
            SqlParam::Integer(_) => "INTEGER",
            SqlParam::BigInt(_) => "BIGINT",
            SqlParam::SmallInt(_) => "SMALLINT",
            SqlParam::Double(_) => "DOUBLE PRECISION",
            SqlParam::Text(_) => "TEXT",
            SqlParam::Varchar(_) => "VARCHAR",
            SqlParam::Boolean(_) => "BOOLEAN",
            SqlParam::Date(_) => "DATE",
            SqlParam::Timestamp(_) => "TIMESTAMP",
            SqlParam::Uuid(_) => "UUID",
            SqlParam::JsonB(_) => "JSONB",
            SqlParam::IntArray(_) => "INTEGER[]",
            SqlParam::TextArray(_) => "TEXT[]",
        }
    }
    pub fn to_sql(&self) -> String {
        match self {
            SqlParam::Integer(Some(v)) => v.to_string(),
            SqlParam::BigInt(Some(v)) => v.to_string(),
            SqlParam::SmallInt(Some(v)) => v.to_string(),
            SqlParam::Double(Some(v)) => v.to_string(),
            SqlParam::Text(Some(v)) | SqlParam::Varchar(Some(v)) => {
                format!("'{}'", v.replace("'", "''"))
            }
            SqlParam::Boolean(Some(v)) => v.to_string(),
            SqlParam::Date(Some(v)) => format!("'{}'", v),
            SqlParam::Timestamp(Some(v)) => format!("'{}'", v),
            SqlParam::Uuid(Some(v)) => format!("'{}'", v),
            SqlParam::JsonB(Some(v)) => format!("'{}'", v.to_string().replace("'", "''")),
            SqlParam::IntArray(Some(v)) => format!(
                "ARRAY[{}]",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            SqlParam::TextArray(Some(v)) => format!(
                "ARRAY[{}]",
                v.iter()
                    .map(|x| format!("'{}'", x.replace("'", "''")))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            _ => "NULL".to_string(),
        }
    }
}

impl DbPool {
    /// Creates a new database connection pool.
    ///
    /// ### Returns
    ///
    /// A `Result` containing the database connection pool if successful, or an error if there is an issue with creating the pool.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let db_pool = DbPool::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        println!("Connecting to database...");
        let pool = Pool::builder().max_size(5).build(manager)?;
        Ok(DbPool { pool })
    }
    /// Executes a stored procedure and returns the results as a typed collection.
    ///
    /// ### Type Parameters
    ///
    /// * `T` - The type that represents a single row in the result set. Must implement `QueryableByName`.
    ///
    /// ### Arguments
    ///
    /// * `proc_name` - The name of the stored procedure to execute
    /// * `params` - A vector of parameter tuples, each containing:
    ///   - Parameter name (string slice)
    ///   - Parameter value (`SqlParam`)
    ///   - Parameter direction (`ParamDirection`)
    ///
    /// ### Returns
    ///
    /// Returns a tuple containing:
    /// * A vector of type `T` representing the result rows
    /// * A HashMap containing the output parameters and their values
    ///
    /// ### Errors
    ///
    /// Returns a `DbError` if:
    /// * Cannot get connection from pool
    /// * Stored procedure execution fails
    /// * Output parameter retrieval fails
    ///
    /// ### Example
    ///
    /// ```rust
    /// use your_crate::{DbPool, SqlParam, ParamDirection};
    /// use serde::Deserialize;
    ///
    /// #[derive(QueryableByName, Debug)]
    /// struct User {
    ///     #[diesel(sql_type = Integer)]
    ///     id: i32,
    ///     #[diesel(sql_type = Text)]
    ///     name: String,
    /// }
    ///
    /// let pool = DbPool::new()?;
    /// let params = vec![
    ///     ("department", SqlParam::Text(Some("IT".to_string())), ParamDirection::In),
    ///     ("count", SqlParam::Integer(None), ParamDirection::Out),
    /// ];
    ///
    /// let (users, outputs) = pool.execute_stored_proc::<User>("get_department_users", params)?;
    /// ```

    pub fn execute_stored_proc<T>(
        &self,
        proc_name: &str,
        params: Vec<(&str, SqlParam, ParamDirection)>,
    ) -> Result<(Vec<T>, HashMap<String, SqlParam>), DbError>
    where
        T: for<'a> diesel::deserialize::QueryableByName<Pg> + 'static,
    {
        let mut conn = self.pool.get()?;
        let mut outputs = HashMap::new();

        let transaction_result =
            conn.transaction(|conn| -> Result<Vec<T>, diesel::result::Error> {
                let mut param_values = Vec::new();
                let mut param_placeholders = Vec::new();
                let mut out_params = Vec::new();

                // Prepare parameters
                for (idx, (name, param, direction)) in params.iter().enumerate() {
                    if *direction == ParamDirection::In {
                        param_values.push(format!("${} := {}", idx + 1, param.to_sql()));
                        param_placeholders.push(format!("${}", idx + 1));
                    } else {
                        out_params.push((*name, param.clone()));
                    }
                }

                // Execute procedure and get results
                let call_statement =
                    format!("CALL {}({});", proc_name, param_placeholders.join(", "));

                // Get the actual result set
                let results = diesel::sql_query(&call_statement).load::<T>(conn)?;

                // Fetch OUT parameters
                for (name, param) in out_params {
                    let query = format!("SELECT {}::{} AS out_param", name, param.get_type_name());

                    let out_value: QueryResult<SqlParam> = diesel::sql_query(&query)
                        .get_result::<OutParam>(conn)
                        .map(|row| match param {
                            SqlParam::Integer(_) => SqlParam::Integer(
                                row.out_param.as_ref().and_then(|v| v.parse().ok()),
                            ),
                            SqlParam::BigInt(_) => SqlParam::BigInt(
                                row.out_param.as_ref().and_then(|v| v.parse().ok()),
                            ),
                            SqlParam::SmallInt(_) => SqlParam::SmallInt(
                                row.out_param.as_ref().and_then(|v| v.parse().ok()),
                            ),
                            SqlParam::Double(_) => {
                                SqlParam::Double(row.out_param.as_ref().and_then(|v| {
                                    v.parse::<rust_decimal::Decimal>().ok().map(DbDecimal::from)
                                }))
                            }
                            SqlParam::Text(_) | SqlParam::Varchar(_) => {
                                SqlParam::Text(row.out_param.clone())
                            }
                            SqlParam::Boolean(_) => SqlParam::Boolean(
                                row.out_param.as_ref().and_then(|v| v.parse().ok()),
                            ),
                            SqlParam::Date(_) => {
                                SqlParam::Date(row.out_param.as_ref().and_then(|v| v.parse().ok()))
                            }
                            SqlParam::Timestamp(_) => SqlParam::Timestamp(
                                row.out_param.as_ref().and_then(|v| v.parse().ok()),
                            ),
                            SqlParam::Uuid(_) => {
                                SqlParam::Uuid(row.out_param.as_ref().and_then(|v| v.parse().ok()))
                            }
                            SqlParam::JsonB(_) => SqlParam::JsonB(
                                row.out_param
                                    .as_ref()
                                    .and_then(|v| serde_json::from_str(v).ok()),
                            ),
                            SqlParam::IntArray(_) => {
                                SqlParam::IntArray(row.out_param.as_ref().and_then(|v| {
                                    let v = v.trim_start_matches('{').trim_end_matches('}');
                                    Some(v.split(',').filter_map(|n| n.parse().ok()).collect())
                                }))
                            }
                            SqlParam::TextArray(_) => {
                                SqlParam::TextArray(row.out_param.as_ref().and_then(|v| {
                                    let v = v.trim_start_matches('{').trim_end_matches('}');
                                    Some(v.split(',').map(|s| s.to_string()).collect())
                                }))
                            }
                        });

                    if let Ok(value) = out_value {
                        outputs.insert(name.to_string(), value);
                    }
                }

                Ok(results)
            });

        transaction_result
            .map(|results| (results, outputs))
            .map_err(|e| {
                DbError::ExecutionError(format!("Error executing stored procedure: {}", e))
            })
    }

    // Executes a stored procedure and returns the results as JSON values.
    ///
    /// This is a convenience wrapper around `execute_stored_proc` that automatically
    /// handles JSON serialization of results.
    ///
    /// ### Arguments
    ///
    /// * `proc_name` - The name of the stored procedure to execute
    /// * `params` - A vector of parameter tuples (name, value, direction)
    ///
    /// ### Returns
    ///
    /// Returns a tuple containing:
    /// * A vector of `serde_json::Value` representing the result rows
    /// * A HashMap containing the output parameters and their values
    ///
    /// ### Example
    ///
    /// ```rust
    /// let params = vec![
    ///     ("status", SqlParam::Text(Some("active".to_string())), ParamDirection::In)
    /// ];
    ///
    /// let (json_results, _) = pool.execute_stored_proc_json("get_active_users", params)?;
    /// for user in json_results {
    ///     println!("User ID: {}", user["id"]);
    ///     println!("Name: {}", user["name"]);
    /// }
    /// ```
    pub fn execute_stored_proc_json(
        &self,
        proc_name: &str,
        params: Vec<(&str, SqlParam, ParamDirection)>,
    ) -> Result<(Vec<JsonValue>, HashMap<String, SqlParam>), DbError> {
        self.execute_stored_proc::<JsonResults>(proc_name, params)
            .map(|(results, outputs)| (results.into_iter().map(|r| r.json).collect(), outputs))
    }

    /// Executes a stored procedure and deserializes the results into the specified type.
    ///
    /// This method is useful when you want to work with strongly-typed structures
    /// but don't want to define Diesel-specific types.
    ///
    /// ### Type Parameters
    ///
    /// * `T` - The type to deserialize results into. Must implement `DeserializeOwned`.
    ///
    /// ### Arguments
    ///
    /// * `proc_name` - The name of the stored procedure to execute
    /// * `params` - A vector of parameter tuples (name, value, direction)
    ///
    /// ### Returns
    ///
    /// Returns a tuple containing:
    /// * A vector of type `T` representing the deserialized results
    /// * A HashMap containing the output parameters and their values
    ///
    /// ### Example
    ///
    /// ```rust
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct UserInfo {
    ///     id: i32,
    ///     name: String,
    ///     email: Option<String>,
    /// }
    ///
    /// let params = vec![
    ///     ("role", SqlParam::Text(Some("admin".to_string())), ParamDirection::In)
    /// ];
    ///
    /// let (users, _) = pool.execute_stored_proc_typed::<UserInfo>("get_users_by_role", params)?;
    /// for user in users {
    ///     println!("User {}: {}", user.id, user.name);
    /// }
    /// ```
    pub fn execute_stored_proc_typed<T>(
        &self,
        proc_name: &str,
        params: Vec<(&str, SqlParam, ParamDirection)>,
    ) -> Result<(Vec<T>, HashMap<String, SqlParam>), DbError>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        self.execute_stored_proc::<JsonResults>(proc_name, params)
            .and_then(|(results, outputs)| {
                let converted: Result<Vec<T>, _> = results
                    .into_iter()
                    .map(|r| serde_json::from_value(r.json))
                    .collect();

                converted
                    .map_err(|e| DbError::ExecutionError(format!("JSON conversion error: {}", e)))
                    .map(|data| (data, outputs))
            })
    }
}
