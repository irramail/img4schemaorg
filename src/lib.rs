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
