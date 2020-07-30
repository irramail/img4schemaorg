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
    <link itemprop=\"contentUrl\" href=\"{url}/{fname}_{aw}_{ah}_{w}_1.jpg\">
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

  if collect_url_and_fname[0].len() < 7 {
    let _ : () = con.set("schemaImgURL", collect_url_and_fname[0])?;
  } else {
    let _ : () = con.set("schemaImgURL", "https://test.domain/upload/images")?;
  };

  let _ : () = con.set("schemaImgFileName", collect_url_and_fname[1])?;
  let _ : () = con.set("schemaImgDescription", collect_url_and_fname[2])?;
  let _ : () = con.set("schemaImgAlt", collect_url_and_fname[3])?;
  let _ : () = con.set("schemaImgMetaName", collect_url_and_fname[4])?;
  let _ : () = con.set("schemaImgMetaDescription", collect_url_and_fname[5])?;
  let _ : () = con.set("schemaImgAspectResolution", collect_url_and_fname[6])?;

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

fn get_width() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImg1Width")
}

fn get_height() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImg1Height")
}

fn get_path(url : &str) -> Result<Url, ParseError> {
  let parsed = Url::parse(url)?;
  Ok(parsed)
}

fn get_meta_name() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgMetaName")
}

fn get_meta_description() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgMetaDescription")
}

fn get_alt() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgAlt")
}

fn get_aspect_resolution() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgAspectResolution")
}

fn gen_srcset(path :&str, fname: &str) -> String {
  let fullpath = format!("{}/{}", &path, &fname);
  format!("srcset=\"{},\n{},\n{},\n{}\"",
          format!("{}_o_320_1.jpg 320w", fullpath),
          format!("{}_o_640_1.jpg 640w", fullpath),
          format!("{}_o_1280_1.jpg 1280w", fullpath),
          format!("{}_o_1920_1.jpg 1920w", fullpath)
  )
}

fn fetch_img(img: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let img = format!("{}", img);

  let _ : () = con.set("schemaImg", img)?;

  let mut echo_hello = Command::new("sh");
  let _status = echo_hello.arg("-c").arg("/home/p6/scripts/schemaImg.sh").status().expect("sh command failed to start");

  let url= get_url().unwrap();
  let path = get_path(url.as_str()).unwrap();
  let fname= get_fname().unwrap();
  let alt = get_alt().unwrap();
  let description =  get_description().unwrap();

  let srcset = gen_srcset(path.path(), fname.as_str());

  let img = format!("<img decoding=\"async\" itemprop=\"contentUrl\" sizes=\"(max-width: 1280px) 640px, 1280px\" {}\nsrc=\"{}/{}_1.jpg\"\nalt=\"{}\">", srcset, path.path(), fname.as_str(), alt);

  let meta_name = format!("<meta itemprop=\"name\" content=\"{}\">", get_meta_name().unwrap());
  let meta_description =  format!("<meta itemprop=\"description\" content=\"{}\">", get_meta_description().unwrap());

  //println!("{} {}", get_width().unwrap(), get_height().unwrap());
  let meta_width_height = format!("<meta itemprop=\"width\" content=\"{}px\">\n<meta itemprop=\"height\" content=\"{}px\">", get_width().unwrap(), get_height().unwrap());

  let ares = get_aspect_resolution().unwrap();

  let mut div_all : String = "".to_string();
  //1:1_640x640,1280x1280,1920x1920;4:3_640x480,1280x960,1920x1440;16:9_640x360,854x480,1280x720,1920x1080
  for ar in ares.split_terminator(';') {
    //1:1_640x640,1280x1280,1920x1920
    let item_ar : Vec<&str> = ar.split_terminator('_').collect();

    //1:1
    let a : Vec<&str> = item_ar[0].split_terminator(':').collect();

    //1 1
    let a_h = a[0];
    let a_w = a[1];

    //640x640,1280x1280,1920x1920
    for res in item_ar[1].split_terminator(',') {
      let item_res : Vec<&str> = res.split_terminator('x').collect();
      let res_w = item_res[0];
      let res_h = item_res[1];

      div_all = format!("{}{}", div_all, div(a_w.parse().unwrap(), a_h.parse().unwrap(), res_w.parse().unwrap(), res_h.parse().unwrap(), url.clone(), fname.clone(), description.clone()));
    }

  }

  println!("{}", div_all);
/*
  let div = format!("{}{}{}{}{}{}{}{}{}{}"
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
*/
  let bwrapper = "<div itemprop=\"image\" itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" class=\"ImageObject_cont\">";
  let ewrapper = "</div>";
  let wrapper = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}", bwrapper, img, meta_name, meta_description, meta_width_height, div_all, ewrapper);

  let _ : () = con.set( "schemaOrg", wrapper)?;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_div() {
    assert_eq!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.com/assets/images/testname_16_9_640_1.jpg\">
    <meta itemprop=\"width\" content=\"640px\">
    <meta itemprop=\"height\" content=\"360px\">
    <meta itemprop=\"name\" content=\"Description in schema block. Размер фото 640x360, отношение сторон 16:9.\">
  </div>
  ", div(16, 9, 640, 360, "https://test.com/assets/images".to_string(), "testname".to_string(), "Description in schema block".to_string()));
  }
}
