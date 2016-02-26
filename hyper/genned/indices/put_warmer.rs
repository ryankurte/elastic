//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn put_name(client: &'a mut hyper::Client, base: String, name: String,
            body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 9 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_name(client: &'a mut hyper::Client, base: String, name: String,
             body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 9 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn put_index_name(client: &'a mut hyper::Client, base: String, index: String,
                  name: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_index_name(client: &'a mut hyper::Client, base: String, index: String,
                   name: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn put_index_type_name(client: &'a mut hyper::Client, base: String,
                       index: String, type: String, name: String,
                       body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_index_type_name(client: &'a mut hyper::Client, base: String,
                        index: String, type: String, name: String,
                        body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmer/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn put_name(client: &'a mut hyper::Client, base: String, name: String,
            body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_name(client: &'a mut hyper::Client, base: String, name: String,
             body: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 10 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn put_index_name(client: &'a mut hyper::Client, base: String, index: String,
                  name: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_index_name(client: &'a mut hyper::Client, base: String, index: String,
                   name: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn put_index_type_name(client: &'a mut hyper::Client, base: String,
                       index: String, type: String, name: String,
                       body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_index_type_name(client: &'a mut hyper::Client, base: String,
                        index: String, type: String, name: String,
                        body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 10 + index.len() +
                                  type.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&type);
    url_fmtd.push_str("/_warmers/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}