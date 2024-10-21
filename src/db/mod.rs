use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use diesel::sql_types::Text;
use std::env;

pub struct DbPool {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(QueryableByName)]
struct Output {
    #[diesel(sql_type = Text)]
    value: String,
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
    /// Executes a stored procedure with the given name and parameters.
    ///
    /// ### Arguments
    ///
    /// * `proc_name` - The name of the stored procedure to execute.
    /// * `params` - A vector of tuples representing the parameters for the stored procedure. Each tuple contains:
    ///     - The name of the parameter.
    ///     - The type of the parameter (not used in the current implementation).
    ///     - An optional value for the parameter, if it is an input parameter.
    ///     - The mode of the parameter, which can be "IN", "OUT", or "INOUT".
    ///
    /// ### Returns
    ///
    /// A `Result` containing a tuple with two vectors:
    /// * The first vector is currently empty and can be modified to return any required results.
    /// * The second vector contains the values of the OUT and INOUT parameters.
    ///
    /// ### Errors
    ///
    /// Returns an error if there is an issue with getting a connection from the pool or executing the SQL queries.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let db_pool = DbPool::new().unwrap();
    /// let params = vec![
    ///     ("param1", "type1", Some("value1"), "IN"),
    ///     ("param2", "type2", None, "OUT"),
    /// ];
    /// let result = db_pool.execute_stored_proc("my_procedure", params);
    /// match result {
    ///     Ok((_, outputs)) => println!("Outputs: {:?}", outputs),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    pub fn execute_stored_proc(
        &self,
        proc_name: &str,
        params: Vec<(&str, &str, Option<impl ToString>, &str)>,
    ) -> Result<(Result<usize, diesel::result::Error>, Vec<String>), Box<dyn std::error::Error>>
    {
        let mut conn = self.pool.get()?;

        // Prepare SQL call for the procedure
        let mut in_params = vec![];
        let mut out_params = vec![];
        let mut call_params = vec![];

        for (name, _type, value, mode) in params {
            match mode {
                "IN" => {
                    if let Some(val) = value {
                        in_params.push(format!("{} := {}", name, val.to_string()));
                    }
                }
                "INOUT" => {
                    if let Some(val) = value {
                        in_params.push(format!("{} := {}", name, val.to_string()));
                    }
                    out_params.push(name);
                }

                "OUT" => {
                    out_params.push(name);
                }
                _ => continue,
            }
            call_params.push(format!("{}", name));
        }

        let in_params_sql = in_params.join(", ");
        let out_params_sql = out_params.join(", ");

        let sql = format!("CALL {}({});", proc_name, in_params_sql);

        let results = sql_query(sql).execute(&mut conn);

        // Now, retrieve the OUT and INOUT parameters (if any)
        let mut outputs = vec![];
        if !out_params_sql.is_empty() {
            let select_sql = format!("SELECT {};", out_params_sql);
            let results: Vec<Output> = sql_query(select_sql).load(&mut conn)?;
            outputs.extend(results.into_iter().map(|output| output.value));
        }

        Ok((results, outputs)) // Return empty results for now, modify as needed
    }
}
