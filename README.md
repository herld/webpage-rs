# Webpage.rs

_Get some info about a webpage_

## Usage

```rust
extern crate webpage;
use webpage::Webpage;

// ...

info = Webpage::from_url("http://www.rust-lang.org/en-US/");

// the HTTP info
let http = info.http.unwrap();

assert_eq!(http.ip, "54.192.129.71".to_string());
assert!(http.headers[0].starts_with("HTTP"));
assert!(http.body.starts_with("<!DOCTYPE html>"));
assert_eq!(http.url, "https://www.rust-lang.org/en-US/".to_string()); // followed redirects
assert_eq!(http.content_type, "text/html".to_string());

// the HTML info
let html = info.html.unwrap();

assert_eq!(html.title.unwrap(), "The Rust Programming Language".to_string());
assert_eq!(html.description.unwrap(), "A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string());
assert_eq!(html.opengraph.og_type, "website".to_string());
```

## All fields

```rust
pub struct Webpage {
    pub http: Option<HTTP>, // info about the HTTP transfer, if any
    pub html: Option<HTML>, // info from the parsed HTML doc, if any
}

pub struct HTTP {
    pub ip: String,
    pub content_type: String,
    pub headers: Vec<String>,
    pub url: String, // effective url
    pub body: String,
}

pub struct HTML {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub text_content: String,

    pub opengraph: Opengraph,
}

pub struct Opengraph {
    pub og_type: String,
    pub properties: HashMap<String, String>,

    pub images: Vec<Object>,
    pub videos: Vec<Object>,
    pub audios: Vec<Object>,
}

pub struct Object {
    pub url: String,
    pub properties: HashMap<String, String>,
}
```
