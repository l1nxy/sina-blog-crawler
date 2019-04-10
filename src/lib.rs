extern crate regex;
extern crate reqwest;
extern crate select;

use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name};
use std::fs;
use std::io::Write;

//获取博文目录
fn get_url() -> Vec<String> {
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
            if url != "http://blog.sina.com.cn/s/blog_7f78b76f0100rq53.html" {//去除掉含有目录正文的那一章
                url_vector.push(url);
            }
        }
    }

    url_vector
}
//得到文章中图片的位置
fn get_image_name(url: &str) -> String {
    let after_split: Vec<&str> = url.rsplit('/').collect();
    after_split[0].to_string()
}

//下载图片到本地
fn download_images(url: &str) {
    println!("downloading {}", url);
    let mut res = reqwest::get(url).expect("Get failed");
    let mut buf: Vec<u8> = vec![];
    let save_name = format!("./html/image/{}.jpg", get_image_name(url));
    buf.clear();
    res.copy_to(&mut buf).expect("response copy to buff failed");
    fs::create_dir_all("./html/image").expect("create failed");
    let mut file = fs::File::create(save_name).expect("create file error");
    file.write_all(buf.as_slice()).expect("Write Failed");
}

//正则匹配出正文中的图片的URL
fn get_images_urls(html: &str) -> Vec<String> {
    let re = Regex::new("real_src=\"(.*?)&").unwrap();
    let mut url_list: Vec<String> = vec![];
    for caps in re.captures_iter(html) {
        let url = caps.get(1).unwrap().as_str();
        url_list.push(url.to_string());
    }
    url_list.dedup_by(|a, b| a == b);
    url_list
}

//得到文章的html与标题
fn get_page_html_and_title(url: &str) -> (String, String) {
    let html_text = reqwest::get(url)
        .expect("Get failed")
        .text()
        .expect("Get text failed");
    let document = Document::from(html_text.as_str());
    let mut html_string = String::new();
    for node in document.find(Class("articalContent")) {
        html_string.push_str(node.html().as_str());
    }
    let mut title = String::new();
    for i in document.find(Class("articalTitle")) {
        title.push_str(i.find(Name("h2")).next().unwrap().text().as_str());
    }
    (html_string, title)
}

//替换html中的图片名字
fn replace_html_image_name(html: &str, url_list: Vec<String>) -> String {
    let mut new_html = String::from(html);
    for i in url_list {
        let image_name = format!("src=\"./image/{}.jpg\" ", get_image_name(i.as_str()));
        let match_name = format!("src=\"http://simg.sinajs.cn/blog7style/images/common/sg_trans.gif\" real_src=\"{}&amp;690\"",i);
        new_html = new_html.replace(match_name.as_str(), image_name.as_str())
    }
    new_html
}

//保存html为文件
fn save_html_to_file(html: &str, name: &str)  {
    let file_path = format!("./html/{}.html", name);
    let mut f = fs::File::create(file_path.as_str()).expect("html file create failed");
    f.write_all(html.as_bytes()).expect("Write failed");
}

fn add_html_block(html: &str) -> String {
    let res = format!("<html><body>{}</body></html>", html);
    res
}

fn add_article_title(html: &str, name: &str) -> String {
    let res = format!("<h1> {} </h1> {}<hr />", name, html);
    res
}

pub fn run() {
    let mut url_list = get_url();
    url_list.reverse();
    let mut html = String::new();
    for url in url_list {
        let (content, title) = get_page_html_and_title(url.as_str());
        let image_urls = get_images_urls(content.as_str());
        for i in image_urls.clone() {
            download_images(i.as_str());
        }
        let new_html = replace_html_image_name(content.as_str(), image_urls);
        let new_html = add_article_title(new_html.as_str(), title.as_str());
        html.push_str(new_html.as_str());
    }
    let html = add_html_block(html.as_str());
    save_html_to_file(html.as_str(), "mahjong");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image_name() {
        let v = get_image_name("http://s2.sinaimg.cn/orignal/7f78b76fgx6BQKiNJaVb1");
        assert_eq!(v, "7f78b76fgx6BQKiNJaVb1");
    }
}
