use std::process::Command;
use url::{Url, ParseError};
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
  format!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"{url}/{fname}_{aw}_{ah}_{h}_1.jpg\">
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
fn get_path(url : &str) -> Result<Url, ParseError> {
  let parsed = Url::parse(url)?;
  Ok(parsed)
}

fn gen_srcset(path :&str, fname: &str) -> String {
  let fullpath = format!("{}/{}", &path, &fname);
  format!("{},\n {},\n {}\n",format!("{}_o_640_1.jpg 640w", fullpath), format!("{}_o_1280_1.jpg 1280w", fullpath), format!("{}_o_1920_1.jpg 1920w", fullpath))
}

fn fetch_img(img: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let img = format!("{}", img);

  let _ : () = con.set("schemaImg", img)?;

  let mut echo_hello = Command::new("sh");
  echo_hello.arg("-c").arg("/home/p6/scripts/schemaImg.sh").spawn().expect("sh command failed to start");

  let url=get_url().unwrap();
  let path = get_path(url.as_str()).unwrap();
  let fname= get_fname().unwrap();
  let description =  get_description().unwrap();

  let srcset = gen_srcset(path.path(), fname.as_str());
println!("{}", srcset);
/*
  <div itemprop="image" itemscope="" itemtype="http://schema.org/ImageObject" class="ImageObject_cont">
    <img decoding="async" itemprop="contentUrl" with="100%" sizes="(max-width: 1280px) 100vw, 1280px" srcset="
/wp-content/uploads/porolon-320-240.jpg 320w,
/wp-content/uploads/porolon-320-320.jpg 320w,
/wp-content/uploads/porolon-426-240.jpg 426w,
/wp-content/uploads/porolon-640-480.jpg 640w,
/wp-content/uploads/porolon-640-640.jpg 640w,
/wp-content/uploads/porolon-854-480.jpg 854w,
/wp-content/uploads/porolon-960-720.jpg 960w,
/wp-content/uploads/porolon-1080-1080.jpg 1080w,
/wp-content/uploads/porolon-1280-720.jpg 1280w,
/wp-content/uploads/porolon-1280-1280.jpg 1280w,
/wp-content/uploads/porolon-1440-1080.jpg 1440w,
/wp-content/uploads/porolon-1920-1080.jpg 1920w"
  src="/wp-content/uploads/porolon-1.jpg"
  alt="Поролон белого цвета, толщина 40мм, рулон.">
    <meta itemprop="name" content="Поролон, рулон, цвет белый, 40мм толщиной.">
    <meta itemprop="description" content="Мебельный поролон или пенополиуретан, продажа в рулонах габаритов 1000мм на 2000мм, толщина 40мм, производство Россия.">
    <meta itemprop="width" content="2272px">
    <meta itemprop="height" content="1704px">
    */


  let div = format!("{}{}{}{}{}{}{}{}{}{}</div>"
                       , div(16, 9, 640, 360, url.clone(), fname.clone(), description.clone())
                       , div(16, 9, 854, 480, url.clone(), fname.clone(), description.clone())
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
