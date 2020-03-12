extern crate redis;

use redis::{Commands};
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params, Error};
use jsonrpc_http_server::{ServerBuilder};

fn parse_arguments (p: Params) -> Result<Vec<String>, Error> {
  let mut result = Vec::new();
  match p {
    Params::Array(array) => {
      for s in &array {
        match s {
          Value::String(s) => result.push(s.clone()),
          _ => return Err(Error::invalid_params("expecting strings"))
        }
      }
    }
    _ => return Err(Error::invalid_params("expecting an array of strings"))
  }
  if result.len() < 1 {
    return Err(Error::invalid_params("missing api key"));
  }

  return Ok(result[0..].to_vec());
}

fn fetch_svg(svg: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let svg = format!("{}", svg);

  let _ : () = con.set("svg", svg.clone())?;
  let _ : () = con.set("backupSvg", svg)?;

  con.get("svg")
}

fn fetch_m4a(m4a: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let m4a = format!("{}", m4a);

  //let _ : () = con.set("stext", "")?;
  let _ : () = con.set("m4a", m4a)?;

  con.get("m4a")
}

fn get_filenames() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("mp4AllFilenames")
}

fn get_new_filenames() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("mp4NewFilenames")
}

fn set_zero() -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("mp4NewFilenames", "")?;

  con.get("mp4NewFilenames")
}

fn delete_by_name(name: &str)  -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let filenames: String = con.get("mp4AllFilenames").unwrap();

  let comma_name = format!(",{}",name).to_string();
  let space_name = format!(" {}",name).to_string();

  let _ : () = con.append("mp4DelFilenames", space_name)?;

  let _ : () = con.set("mp4AllFilenames", filenames.replace(&comma_name, ""))?;

  let _ : () = con.set("mp4NewFilenames", "")?;
  con.get("mp4AllFilenames")
}

fn set_text(text: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("text", text)?;
  let _ : () = con.set("stext", text)?;

  con.get("text")
}

fn get_text() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("btext")
}

fn get_stext() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("stext")
}

fn set_zero_text() -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("btext", "")?;
  let _ : () = con.set("stext", "")?;
  let _ : () = con.set("sec", "0.25")?;
  let _ : () = con.set("voice", "Samantha")?;
  let _ : () = con.set("color", "000000")?;
  let _ : () = con.set("ypercent", "10")?;

  con.get("btext")
}

fn set_sec(sec: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("sec", sec)?;

  con.get("sec")
}

fn set_voice(voice: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("voice", voice)?;

  con.get("voice")
}

fn set_color(color: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("color", color)?;

  con.get("color")
}

fn set_ypercent(ypercent: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("ypercent", ypercent)?;

  con.get("ypercent")
}

fn set_bgcounter(bgcounter: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("bgcounter", bgcounter)?;

  con.get("bgcounter")
}

fn set_repeat() -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let old_svg : String = con.get("backupSvg").unwrap();

  let _ : () = con.set("svg", &old_svg)?;

  con.get("svg")
}

fn get_sec() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("sec")
}

fn get_voice() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("voice")
}

fn get_color() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("color")
}

fn get_ypercent() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("ypercent")
}

fn main() {
  let mut io = IoHandler::new();

  let _ = set_zero_text();

  io.add_method("get_data",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = fetch_svg( &w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("get_data_mp3",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = fetch_m4a( &w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("get_files",  | _params | {
    let filenames = get_filenames().unwrap();
    Ok(Value::String(filenames))
  });

  io.add_method("get_new_files",  | _params | {
    let filenames = get_new_filenames().unwrap();
    let _ = set_zero();

    Ok(Value::String(filenames))
  });

  io.add_method("delete",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = delete_by_name(&w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set_text",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = set_text(&w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("get_text",  | _params | {
    let text = get_text().unwrap();

    Ok(Value::String(text))
  });

  io.add_method("get_stext",  | _params | {
    let text = get_stext().unwrap();

    Ok(Value::String(text))
  });

  io.add_method("set_sec",  move |params: Params| {
    let secs = parse_arguments(params)?;
    let _ = set_sec(&secs[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set_voice",  move |params: Params| {
    let voices = parse_arguments(params)?;
    let _ = set_voice(&voices[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set_color",  move |params: Params| {
    let color = parse_arguments(params)?;
    let _ = set_color(&color[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set_ypercent",  move |params: Params| {
    let ypercent = parse_arguments(params)?;
    let _ = set_ypercent(&ypercent[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set_bgcounter",  move |params: Params| {
    let bgcounter = parse_arguments(params)?;
    let _ = set_bgcounter(&bgcounter[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set_repeat", | _params | {
    let _ = set_repeat();

    Ok(Value::String("".to_string()))
  });

  io.add_method("get_sec",  | _params | {
    let sec = get_sec().unwrap();

    Ok(Value::String(sec))
  });

  io.add_method("get_voice",  | _params | {
    let voice = get_voice().unwrap();

    Ok(Value::String(voice))
  });

  io.add_method("get_color",  | _params | {
    let color = get_color().unwrap();

    Ok(Value::String(color))
  });

  io.add_method("get_ypercent",  | _params | {
    let ypercent = get_ypercent().unwrap();

    Ok(Value::String(ypercent))
  });

  let server = ServerBuilder::new(io)
    .threads(3)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .unwrap();

  server.wait();
}
