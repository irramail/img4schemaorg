use std::process::Command;
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

fn div(aw:i32, ah:i32, w:i32, h:i32, url: String, fname: String, description: String) -> String {
  format!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;>
    <link itemprop=\"contentUrl\" href=\"{url}/{fname}_{aw}_{ah}_{h}.jpg\">
    <meta itemprop=\"width\" content=\"{w}px\">
    <meta itemprop=\"height\" content=\"{h}px\">
    <meta itemprop=\"name\" content=\"{description}. Размер фото {w}x{h}, отношение сторон {aw}:{ah}.\">
  </div>
  ", aw=aw.to_string(), ah=ah.to_string(), w=w.to_string(), h=h.to_string(), url = url, fname = fname, description = description)
}

fn set_url_and_fname(url_and_fname: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let collect_url_and_fname: Vec<&str> = url_and_fname.split("|").collect();

  let _ : () = con.set("schemaImgURL", collect_url_and_fname[0])?;
  let _ : () = con.set("schemaImgFileName", collect_url_and_fname[1])?;
  let _ : () = con.set("schemaImgDescription", collect_url_and_fname[2])?;

  con.get("schemaImgFileName")
}

fn get_url() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgURL")
}

fn get_fname() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgFileName")
}

fn get_description() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgDescription")
}

fn fetch_img(img: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let img = format!("{}", img);

  let _ : () = con.set("schemaImg", img)?;

  let mut echo_hello = Command::new("sh");
  echo_hello.arg("-c").arg("/home/p6/scripts/schemaImg.sh").spawn().expect("sh command failed to start");

  let url=get_url().unwrap();
  let fname=get_fname().unwrap();
  let description =  get_description().unwrap();

  let div = format!("{}{}{}{}{}{}{}{}{}{}"
                       , div(16, 9, 640, 360, url.clone(), fname.clone(), description.clone())
                       , div(16, 9, 853, 480, url.clone(), fname.clone(), description.clone())
                       , div(16, 9, 1280, 720, url.clone(), fname.clone(), description.clone())
                       , div(16, 9, 1920, 1080, url.clone(), fname.clone(), description.clone())
                       , div(4, 3, 640, 480, url.clone(), fname.clone(), description.clone())
                       , div(4, 3, 1280, 960, url.clone(), fname.clone(), description.clone())
                       , div(4, 3, 1920, 1440, url.clone(), fname.clone(), description.clone())
                       , div(1, 1, 640, 640, url.clone(), fname.clone(), description.clone())
                       , div(1, 1, 1280, 1280, url.clone(), fname.clone(), description.clone())
                       , div(1, 1, 1920, 1920, url, fname, description));

  let _ : () = con.set( "schemaOrg", div)?;

  con.get("schemaImg")
}

fn exists_img() -> redis::RedisResult<bool> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.exists("schemaImg")
}

fn get_schema_org() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaOrg")
}

fn set_first_run() -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.del("schemaImg")?;

  con.get("mayBeSomeKey")
}

fn main() {
  let mut io = IoHandler::new();

  let _ = set_first_run();

  io.add_method("set_img",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = fetch_img( &w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = set_url_and_fname( &w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("exists_img",  | _params | {
    let bool = exists_img().unwrap().to_string();
    Ok(Value::String(bool))
  });

  io.add_method("get_schema_org",  | _params | {
    let schema = get_schema_org().unwrap();
    Ok(Value::String(schema))
  });

  let server = ServerBuilder::new(io)
  .threads(3)
  .start_http(&"127.0.0.1:3033".parse().unwrap())
  .unwrap();

  server.wait();
}
