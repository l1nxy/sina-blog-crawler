extern crate regex;
extern crate reqwest;
extern crate select;
use regex::Regex;
use select::document::Document;
use select::predicate::Class;

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
            url_vector.push(url);
        }
        println!("{}", url_vector.iter().count());
    }

    //    for url in url_vector{
    //        println!("{}",url);
    //    }
    url_vector
}

fn main() -> Result<(), Box<std::error::Error>> {
    //id="sina_keyword_ad_area2" class="articalContent   "
    let url_list = get_url();
    let url = url_list.get(0).unwrap();
    println!("{}", url);
    let html_text = reqwest::get(url.as_str())?.text()?;
    //println!("{}",html_text);
    let document = Document::from(html_text.as_str());
    for node in document.find(Class("articalContent")) {
        println!("{:#?}", node.html());
    }
    //    for url in url_list{
    //
    //
    //    }
    Ok(())
}
