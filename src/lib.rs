use url::{Url, ParseError};
extern crate redis;

use redis::{Commands};

pub fn div(aw:i32, ah:i32, w:i32, h:i32, url: String, fname: String, description: String) -> String {
  format!("<div itemscope=\"\" itemtype=\"http://schema.org/ImageObject\" itemprop=\"thumbnail\" style=\"display:none;\">
    <link itemprop=\"contentUrl\" href=\"{url}/{fname}_{aw}_{ah}_{w}_1.jpg\">
    <meta itemprop=\"width\" content=\"{w}px\">
    <meta itemprop=\"height\" content=\"{h}px\">
    <meta itemprop=\"name\" content=\"{description}. Размер фото {w}x{h}, отношение сторон {aw}:{ah}.\">
  </div>
  ", aw=aw.to_string(), ah=ah.to_string(), w=w.to_string(), h=h.to_string(), url = url, fname = fname, description = description)
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

fn get_alt() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgAlt")
}

fn get_description() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgDescription")
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

fn get_aspect_resolution() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("schemaImgAspectResolution")
}

pub fn div_creator() -> String {
  let url= get_url().unwrap();
  let path = get_path(url.as_str()).unwrap();
  let fname= get_fname().unwrap();
  let alt = get_alt().unwrap();
  let description =  get_description().unwrap();

  let srcset = gen_srcset(path.path(), fname.as_str());

  let img = format!("<img decoding=\"async\" itemprop=\"contentUrl\" sizes=\"(max-width: 1280px) 320px, 640px, 1280px\" {}\nsrc=\"{}/{}_1.jpg\"\nalt=\"{}\">", srcset, path.path(), fname.as_str(), alt);

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
    let a_w = a[0];
    let a_h = a[1];

    //640x640,1280x1280,1920x1920
    for res in item_ar[1].split_terminator(',') {
      let item_res : Vec<&str> = res.split_terminator('x').collect();
      let res_w = item_res[0];
      let res_h = item_res[1];

      div_all = format!("{}{}", div_all, div(a_w.parse().unwrap(), a_h.parse().unwrap(), res_w.parse().unwrap(), res_h.parse().unwrap(), url.clone(), fname.clone(), description.clone()));
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
  ", div(16, 9, 640, 360, "https://test.com/assets/images".to_string(), "testname".to_string(), "Description in schema block".to_string()));
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
