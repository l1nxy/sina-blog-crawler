
extern crate regex;
extern crate reqwest;
extern crate select;

use regex::Regex;
use select::document::Document;
use select::predicate::Class;
use std::fs::File;
use std::io::Write;

pub fn get_url() -> Vec<String> {
    let re = Regex::new("target=\"_blank\" href=\"(.*?)\">").unwrap();
    let mut url_vector = Vec::new();
    for i in 1..=3 {
        let url_of_table = format!(
            "http://blog.sina.com.cn/s/articlelist_2138617711_0_{}.html",
            i
        );
        let html_text = reqwest::get(url_of_table.as_str()).unwrap().text().unwrap();
        for caps in re.captures_iter(html_text.as_str()) {
            let url = String::from(caps.get(1).unwrap().as_str());
            url_vector.push(url);
        }
        println!("{}", url_vector.iter().count());
    }

    url_vector
}

pub fn get_image_name(url: &str) -> String {
    let after_split: Vec<&str> = url.rsplit('/').collect();
    after_split.get(0).unwrap().to_string()
}

pub fn download_images(url: Vec<String>) -> Result<(), Box<std::error::Error>> {
    let mut res = reqwest::get("http://s2.sinaimg.cn/orignal/7f78b76fgx6BQKiNJaVb1")?;
    let mut buf: Vec<u8> = vec![];
    buf.clear();
    res.copy_to(&mut buf);
    let mut file = File::create("test.jpg").expect("create file error");
    file.write_all(buf.as_slice())?;
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_image_name() {
        let v = get_image_name("http://s2.sinaimg.cn/orignal/7f78b76fgx6BQKiNJaVb1");
        assert_eq!(v, "7f78b76fgx6BQKiNJaVb1");
    }
}
