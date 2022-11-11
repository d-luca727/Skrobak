use std::{collections::HashMap, str::FromStr};

use scraper::{node::Element, Html, Selector};

use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, ClientBuilder,
};

pub type Headers = HashMap<String, String>;

//struct Skrobak
#[derive(Debug)]
pub struct Skrobak {
    client: Client,
}

//default
impl Default for Skrobak {
    fn default() -> Self {
        let headers: Headers = HashMap::new();
        //Self::insert_default_headers(&mut headers, Default::default());

        Self {
            client: ClientBuilder::default()
                .default_headers(map_to_reqwest_headers(&headers))
                .build()
                .unwrap(),
        }
    }
}

//impl Skrobak

impl Skrobak {
    pub async fn parse_web_page_from_url(&self, url: &str) -> Result<Html, reqwest::Error> {
        //let response = reqwest::get(url).await?.text().await?;

        let response = self.client.get(url).send().await?.text().await?;

        Ok(Html::parse_document(&response))
    }
}

//UTILS

pub fn map_to_reqwest_headers(map: &Headers) -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    for (key, value) in map {
        headers.insert(HeaderName::from_str(key).unwrap(), value.parse().unwrap());
    }
    headers
}

//retrieves html document by a given url
pub async fn parse_web_page_from_url(url: &str) -> Result<Html, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;

    Ok(Html::parse_document(&response))
}

//TODO
//types to reimplement: scraper::Html, reqwest::Error, scraper::Selector.

//skrobak html

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkrobarHtml {
    html: Html,
}

impl SkrobarHtml {
    fn new() -> Self {
        Self {
            html: Html::new_document(),
        }
    }
}

//TODOs

// 1. parse returning a list of inner html Strings by a given selector
// 2. parse returning an inner html elements by a given selector
//      2.1 create Element struct with relevant data
// 3. create specific functions
//      3.x (i.e. utility function that retrieves inner propriety of a form or inner values of a table)

//prototype todo 1.

//returns inner html as String
pub async fn parse_by_html_selector(
    url: &str,
    selector: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let web_document = parse_web_page_from_url(&url).await?;

    let selector = Selector::parse(selector).unwrap();

    Ok(web_document
        .select(&selector)
        .map(|e| e.inner_html())
        .collect::<Vec<String>>())
}

//prototype todo 2.

//returns inner Element
pub async fn parse_by_html_selector_returning_html_element(
    url: &str,
    selector: &str,
) -> Result<Vec<Element>, reqwest::Error> {
    let web_document = parse_web_page_from_url(&url).await?;

    let selector = Selector::parse(selector).unwrap();

    Ok(web_document
        .select(&selector)
        .map(|e| e.value().to_owned())
        .collect::<Vec<Element>>())
}

//prototype todo 3.
//...

#[cfg(test)]
mod tests {

    use super::*;
    use httpmock::prelude::*;
    use scraper::Html;

    #[tokio::test]
    async fn parse_web_page_from_url_test() {
        let html = r#"
            <!DOCTYPE html>
            <meta charset="utf-8">
            <title>Hello, world!</title>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.path("/to_scrape");
            then.status(200)
                .header("content-type", "text/html")
                .body(&html);
        });
        let url = format!("{}/to_scrape", &server.base_url());

        let web_document = parse_web_page_from_url(&url).await.unwrap();

        let document = Html::parse_document(html);
        assert_eq!(web_document, document);
    }

    #[tokio::test]
    async fn parse_web_page_from_url_and_selector_test() {
        let html = r#"
        <ul>
            <li>Foo</li>
            <li>Bar</li>
            <li>Baz</li>
        </ul>
        "#;

        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.path("/to_scrape");
            then.status(200)
                .header("content-type", "text/html")
                .body(&html);
        });
        let url = format!("{}/to_scrape", &server.base_url());

        let vec_server = vec!["Foo".to_owned(), "Bar".to_owned(), "Baz".to_owned()];
        let mut vec_client = vec![];

        for el in parse_by_html_selector(&url, "li").await.unwrap().iter() {
            let string = el.clone();
            vec_client.push(string);
        }

        assert_eq!(vec_server, vec_client);
    }
}
