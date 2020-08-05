use url::{Url, ParseError};
use jsonrpc_http_server::jsonrpc_core::{Value, Params, Error};
use std::process::Command;

extern crate redis;

use redis::{Commands};

pub fn parse_arguments (p: Params) -> Result<Vec<String>, Error> {
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

fn run_script() {
  let mut echo_hello = Command::new("sh");
  let _status = echo_hello.arg("-c").arg("scripts/schemaImg.sh").status().expect("sh command failed to start");
}

pub fn fetch_img(img: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;
  let img = format!("{}", img);

  let _ : () = con.set("schemaImg", img.clone())?;
  let _ : () = con.set("backupSchemaImg", img)?;

  run_script();

  let _ : () = con.set( "schemaOrg", div_creator(props().unwrap().as_str(), get_width().unwrap().as_str(), get_height().unwrap().as_str()))?;

  con.get("schemaImg")
}

pub fn exists_img() -> redis::RedisResult<bool> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.exists("schemaImg")
}

fn backup_schema_img() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("backupSchemaImg")
}

pub fn retry() -> redis::RedisResult<bool> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set( "schemaImg", backup_schema_img().unwrap())?;

  run_script();
  let _ : () = con.set( "schemaOrg", div_creator(props().unwrap().as_str(), get_width().unwrap().as_str(), get_height().unwrap().as_str()))?;

  con.exists("backupSchemaImg")
}

fn div(aw:i32, ah:i32, w:i32, h:i32, url: &str, file_name: &str, description: &str) -> String {
  format!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"{url}/{file_name}_{aw}_{ah}_{w}_1.jpg\">
    <meta itemprop=\"width\" content=\"{w}px\">
    <meta itemprop=\"height\" content=\"{h}px\">
    <meta itemprop=\"name\" content=\"{description}. Размер фото {w}x{h}, отношение сторон {aw}:{ah}.\">
  </div>
  ", aw=aw.to_string(), ah=ah.to_string(), w=w.to_string(), h=h.to_string(), url = url, file_name = file_name, description = description)
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

fn get_path(url : &str) -> Result<Url, ParseError> {
  let parsed = Url::parse(url)?;
  Ok(parsed)
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

pub fn all_settings(all_settings: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("schemaImgAllSettings", all_settings)?;

  con.get("schemaImgAllSettings")
}

fn props() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgAllSettings")
}

fn div_creator(tmp_props: &str, width: &str, height: &str) -> String {
  let props: Vec<&str> = tmp_props.split("|").collect();

  let mut url = "https://test.domain/upload/images";

  if props[0].len() >= 9 {
    url = props[0].trim_end_matches('/');
  }

  let path = get_path(url).unwrap();
  let file_name= props[1];
  let description =  props[2];
  let alt = props[3];
  let meta_name = format!("<meta itemprop=\"name\" content=\"{}\">", props[4]);
  let meta_description =  format!("<meta itemprop=\"description\" content=\"{}\">", props[5]);
  let ares = props[6];

  let srcset = gen_srcset(path.path(), file_name);

  let img = format!("<img decoding=\"async\" itemprop=\"contentUrl\" sizes=\"(max-width: 1280px) 320px, 640px, 1280px\" {}\nsrc=\"{}/{}_1.jpg\"\nalt=\"{}\">", srcset, path.path(), file_name, alt);

  let meta_width_height = format!("<meta itemprop=\"width\" content=\"{}px\">\n<meta itemprop=\"height\" content=\"{}px\">", width, height);

  let bwrapper = "<div itemprop=\"image\" itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" class=\"ImageObject_cont\">";
  let ewrapper = "</div>";

  let mut div_all : String = "".to_string();
  //1:1_640x640,1280x1280,1920x1920;4:3_640x480,1280x960,1920x1440;16:9_640x360,854x480,1280x720,1920x1080
  for ar in ares.split_terminator(';') {
    //1:1_640x640,1280x1280,1920x1920
    let item_ar : Vec<&str> = ar.split_terminator('_').collect();

    //1:1
    let a : Vec<&str> = item_ar[0].split_terminator(':').collect();

    //1 1
    let a_w = a[0];
    let a_h = a[1];

    //640x640,1280x1280,1920x1920
    for res in item_ar[1].split_terminator(',') {
      let item_res : Vec<&str> = res.split_terminator('x').collect();
      let res_w = item_res[0];
      let res_h = item_res[1];

      div_all = format!("{}{}", div_all, div(a_w.parse().unwrap(), a_h.parse().unwrap(), res_w.parse().unwrap(), res_h.parse().unwrap(), url, file_name, description));
    }
  }

  format!("{}\n{}\n{}\n{}\n{}\n{}\n{}", bwrapper, img, meta_name, meta_description, meta_width_height, div_all, ewrapper)
}

pub fn get_schema_org() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaOrg")
}

pub fn set_first_run() -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.del("schemaImg")?;

  con.get("mayBeSomeKey")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success_div() {
    assert_eq!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.com/assets/images/testname_16_9_640_1.jpg\">
    <meta itemprop=\"width\" content=\"640px\">
    <meta itemprop=\"height\" content=\"360px\">
    <meta itemprop=\"name\" content=\"Description in schema block. Размер фото 640x360, отношение сторон 16:9.\">
  </div>
  ", div(16, 9, 640, 360, "https://test.com/assets/images", "testname", "Description in schema block"));
  }

  #[test]
  fn success_gen_srcset() {
    assert_eq!("srcset=\"/path/file_name_o_320_1.jpg 320w,
/path/file_name_o_640_1.jpg 640w,
/path/file_name_o_1280_1.jpg 1280w,
/path/file_name_o_1920_1.jpg 1920w\"",
    gen_srcset("/path", "file_name"));
  }

  #[test]
  fn success_div_creator() {
    assert_eq!("<div itemprop=\"image\" itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" class=\"ImageObject_cont\">
<img decoding=\"async\" itemprop=\"contentUrl\" sizes=\"(max-width: 1280px) 320px, 640px, 1280px\" srcset=\"/assets/upload/test_div_all_o_320_1.jpg 320w,
/assets/upload/test_div_all_o_640_1.jpg 640w,
/assets/upload/test_div_all_o_1280_1.jpg 1280w,
/assets/upload/test_div_all_o_1920_1.jpg 1920w\"
src=\"/assets/upload/test_div_all_1.jpg\"
alt=\"test_div_all alt\">
<meta itemprop=\"name\" content=\"test_div_all meta name\">
<meta itemprop=\"description\" content=\"test_div_all meta description\">
<meta itemprop=\"width\" content=\"2048px\">
<meta itemprop=\"height\" content=\"1536px\">
<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_1_1_320_1.jpg\">
    <meta itemprop=\"width\" content=\"320px\">
    <meta itemprop=\"height\" content=\"320px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 320x320, отношение сторон 1:1.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_1_1_640_1.jpg\">
    <meta itemprop=\"width\" content=\"640px\">
    <meta itemprop=\"height\" content=\"640px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 640x640, отношение сторон 1:1.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_1_1_1280_1.jpg\">
    <meta itemprop=\"width\" content=\"1280px\">
    <meta itemprop=\"height\" content=\"1280px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 1280x1280, отношение сторон 1:1.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_1_1_1920_1.jpg\">
    <meta itemprop=\"width\" content=\"1920px\">
    <meta itemprop=\"height\" content=\"1920px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 1920x1920, отношение сторон 1:1.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_4_3_320_1.jpg\">
    <meta itemprop=\"width\" content=\"320px\">
    <meta itemprop=\"height\" content=\"240px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 320x240, отношение сторон 4:3.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_4_3_640_1.jpg\">
    <meta itemprop=\"width\" content=\"640px\">
    <meta itemprop=\"height\" content=\"480px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 640x480, отношение сторон 4:3.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_4_3_1280_1.jpg\">
    <meta itemprop=\"width\" content=\"1280px\">
    <meta itemprop=\"height\" content=\"960px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 1280x960, отношение сторон 4:3.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_4_3_1920_1.jpg\">
    <meta itemprop=\"width\" content=\"1920px\">
    <meta itemprop=\"height\" content=\"1440px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 1920x1440, отношение сторон 4:3.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_16_9_320_1.jpg\">
    <meta itemprop=\"width\" content=\"320px\">
    <meta itemprop=\"height\" content=\"180px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 320x180, отношение сторон 16:9.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_16_9_640_1.jpg\">
    <meta itemprop=\"width\" content=\"640px\">
    <meta itemprop=\"height\" content=\"360px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 640x360, отношение сторон 16:9.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_16_9_854_1.jpg\">
    <meta itemprop=\"width\" content=\"854px\">
    <meta itemprop=\"height\" content=\"480px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 854x480, отношение сторон 16:9.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_16_9_1280_1.jpg\">
    <meta itemprop=\"width\" content=\"1280px\">
    <meta itemprop=\"height\" content=\"720px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 1280x720, отношение сторон 16:9.\">
  </div>
  <div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"https://test.domain/assets/upload/test_div_all_16_9_1920_1.jpg\">
    <meta itemprop=\"width\" content=\"1920px\">
    <meta itemprop=\"height\" content=\"1080px\">
    <meta itemprop=\"name\" content=\"test_div_all thumb description. Размер фото 1920x1080, отношение сторон 16:9.\">
  </div>
  \n</div>",
    div_creator("https://test.domain/assets/upload|test_div_all|test_div_all thumb description|test_div_all alt|test_div_all meta name|test_div_all meta description|1:1_320x320,640x640,1280x1280,1920x1920;4:3_320x240,640x480,1280x960,1920x1440;16:9_320x180,640x360,854x480,1280x720,1920x1080", "2048", "1536"));
  }
}
