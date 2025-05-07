pub mod model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("postgres error: {0}")]
  PgError(#[from] sqlx::Error),
}

pub struct Db(sqlx::postgres::PgPool);

impl Db {
  pub async fn new(connstr: impl AsRef<str>) -> Result<Self> {
    Ok(Db(sqlx::PgPool::connect(connstr.as_ref()).await?))
  }

  pub async fn insert_trade(&self, trade: model::Trade) -> Result<()> {
    sqlx::query_as!(
      model::Trade,
      "INSERT INTO trades 
        ( ask, 
          ask_qty, 
          bid, 
          bid_qty, 
          change, 
          change_pct, 
          high, 
          last, 
          low, 
          symbol, 
          volume, 
          vwap, 
          time ) 
        VALUES 
          (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", 
      trade
    );

    Ok(())
  }
}

