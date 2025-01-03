use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
  // register the table
  let ctx = SessionContext::new();
  ctx.register_csv("dogs", "/home/terje/utvikling/rust/datafusion/dogs.csv", CsvReadOptions::new()).await?;

  // create a plan to run a SQL query
  let df = ctx.sql("SELECT * from dogs").await?;

  // execute and print results
  df.show().await?;
  Ok(())
}