//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn put(client: &'a mut hyper::Client, base: String, body: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 10);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_settings");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn put_index(client: &'a mut hyper::Client, base: String, index: String,
             body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_settings");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}