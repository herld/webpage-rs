use curl::easy::Easy;
use std::time::Duration;

pub struct HTTP {
    pub ip: String,
    pub content_type: String,
    pub headers: Vec<String>,
    pub url: String, // effective url
    pub body: String,
}

impl HTTP {
    pub fn fetch(url: &str) -> Self {
        let mut handle = Easy::new();

        // configure
        handle.timeout(Duration::from_secs(10)).unwrap();
        handle.follow_location(true).unwrap();
        handle.max_redirections(5).unwrap();
        handle.useragent("Webpage - rust crate").unwrap();

        handle.url(url).unwrap();

        let mut headers = Vec::new();
        let mut body = Vec::new();
        {
            let mut transfer = handle.transfer();
            transfer.header_function(|new_data| {
                let header = String::from_utf8_lossy(new_data).into_owned();

                // clear list on redirects
                if header.starts_with("HTTP/") {
                    headers = Vec::new();
                }

                headers.push(header);
                true
            }).unwrap();

            transfer.write_function(|new_data| {
                body.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        let body = String::from_utf8_lossy(&body).into_owned();

        HTTP {
            ip: handle.primary_ip().unwrap().unwrap().to_string(),
            content_type: handle.content_type().unwrap().unwrap().to_string(),
            url: handle.effective_url().unwrap().unwrap().to_string(),

            headers,
            body: body,
        }
    }
}
