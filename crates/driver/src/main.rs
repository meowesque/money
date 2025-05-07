use futures_util::{SinkExt, StreamExt};
use kraken_api::{Request, Response};
use tokio_tungstenite::{
  connect_async,
  tungstenite::{Message, Utf8Bytes},
};

struct Client {}

#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  let (mut socket, response) = connect_async("wss://ws.kraken.com/v2")
    .await
    .expect("Can't connect");

  println!("Connected to the server");
  println!("Response HTTP code: {}", response.status());
  println!("Response contains the following headers:");
  for (header, _value) in response.headers() {
    println!("* {header}");
  }

  let (mut write, read) = socket.split();

  write
    .send(tokio_tungstenite::tungstenite::Message::Text(
      Utf8Bytes::from(
        serde_json::to_string(&Request {
          request_id: Some(123),
          data: kraken_api::RequestData::Subscribe(kraken_api::RequestParams::Ticker {
            symbol: vec!["LTC/BTC".to_owned()],
            event_trigger: kraken_api::EventTrigger::Trades,
            snapshot: false,
          }),
        })
        .unwrap(),
      ),
    ))
    .await
    .unwrap();

  read
    .for_each(async |msg| match msg {
      Ok(Message::Text(bytes)) => {
        let str = bytes.as_str();
        println!("{}", str);
        println!("{:?}", serde_json::from_str::<Response>(str).unwrap());
      }
      e => eprintln!("{e:?}"),
    })
    .await;
}
