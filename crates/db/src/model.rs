#[derive(sqlx::FromRow)]
pub struct Trade {
  pub ask: f32,
  pub ask_qty: f32,
  pub bid: f32,
  pub bid_qty: f32,
  pub change: f32,
  pub change_pct: f32,
  pub high: f32,
  pub last: f32,
  pub low: f32,
  pub symbol: String,
  pub volume: f32,
  pub vwap: f32
}