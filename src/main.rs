extern crate redis;

use redis::{Commands};
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value};
use jsonrpc_http_server::{ServerBuilder};

fn get_data() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://192.168.9.9/")?;
  let mut con = client.get_connection()?;

  con.get("all")
}

fn main() {
  let mut io = IoHandler::new();

  io.add_method("get_data",  | _params | {
    let data = get_data().unwrap();
    Ok(Value::String((format!("[{{\"t\":0, \"a\":\"0\", \"c\":0}}, {} ,{{\"t\":0, \"a\":\"0\", \"c\":0}}]", &data[..data.len()-1])).to_string()))
  });

  let server = ServerBuilder::new(io)
    .threads(3)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .unwrap();

  server.wait();
}
