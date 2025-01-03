use std::env;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
  // register the table
  let ctx = SessionContext::new();
  ctx.register_csv("tabell", &file_path, CsvReadOptions::new()).await?;

  // create a plan to run a SQL query
  let df = ctx.sql("SELECT * from tabell").await?;

  // execute and print results
  df.show().await?;
  Ok(())
}