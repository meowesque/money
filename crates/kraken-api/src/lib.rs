use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EventTrigger {
  Bbo,
  Trades,
}

#[derive(Serialize)]
#[serde(tag = "channel", rename_all = "snake_case")]
pub enum RequestParams {
  Ticker {
    symbol: Vec<String>,
    event_trigger: EventTrigger,
    snapshot: bool,
  },
}

#[derive(Serialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum RequestData {
  Subscribe(RequestParams),
}

#[derive(Serialize)]
pub struct Request {
  #[serde(rename = "req_id")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub request_id: Option<i64>,
  #[serde(flatten)]
  pub data: RequestData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
  Snapshot,
  Update,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TickerResponse {
  pub ask: f32,
  #[serde(rename = "ask_qty")]
  pub ask_quantity: f32,
  pub bid: f32,
  #[serde(rename = "bid_qty")]
  pub bid_quantity: f32,
  pub change: f32,
  #[serde(rename = "change_pct")]
  pub change_percent: f32,
  pub high: f32,
  pub last: f32,
  pub low: f32,
  pub symbol: String,
  pub volume: f32,
  pub vwap: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusResponse {
  pub version: String,
  pub system: String,
  pub api_version: String,
  pub connection_id: u64,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "channel", content = "data", rename_all = "snake_case")]
pub enum ResponseData {
  Status(Vec<StatusResponse>),
  Heartbeat,
  Ticker(Vec<TickerResponse>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
  Response {
    r#type: Option<ResponseType>,
    #[serde(flatten)]
    data: ResponseData,
  },
  Confirmation {
    method: String,
    #[serde(rename = "req_id")]
    request_id: i64,
    result: ConfirmationResult,
    success: bool,
    time_in: DateTime<Utc>,
    time_out: DateTime<Utc>,
  },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "channel", rename_all = "snake_case")]
pub enum ConfirmationResult {
  Ticker {
    event_trigger: EventTrigger,
    snapshot: bool,
    symbol: String,
  },
}

#[cfg(test)]
mod test {
  use chrono::Utc;

  #[test]
  fn test() {
  }
}
