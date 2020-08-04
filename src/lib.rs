use url::{Url, ParseError};
extern crate redis;

use redis::{Commands};

pub fn div(aw:i32, ah:i32, w:i32, h:i32, url: &str, file_name: &str, description: &str) -> String {
  format!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"{url}/{file_name}_{aw}_{ah}_{w}_1.jpg\">
    <meta itemprop=\"width\" content=\"{w}px\">
    <meta itemprop=\"height\" content=\"{h}px\">
    <meta itemprop=\"name\" content=\"{description}. Размер фото {w}x{h}, отношение сторон {aw}:{ah}.\">
  </div>
  ", aw=aw.to_string(), ah=ah.to_string(), w=w.to_string(), h=h.to_string(), url = url, file_name = file_name, description = description)
}

pub fn gen_srcset(path :&str, fname: &str) -> String {
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

pub fn div_creator() -> String {

  let tmp_props = props().unwrap();

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

  let meta_width_height = format!("<meta itemprop=\"width\" content=\"{}px\">\n<meta itemprop=\"height\" content=\"{}px\">", get_width().unwrap(), get_height().unwrap());

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

  let bwrapper = "<div itemprop=\"image\" itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" class=\"ImageObject_cont\">";
  let ewrapper = "</div>";
  format!("{}\n{}\n{}\n{}\n{}\n{}\n{}", bwrapper, img, meta_name, meta_description, meta_width_height, div_all, ewrapper)
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
}
