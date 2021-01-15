use log::{debug};
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestConfig {
    pub url: String,
    pub body: Option<String>,
    pub method: Option<String>,
}

pub fn load_request_definition(file: &str) -> Result<RequestConfig, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(file)?;
    let request_config: RequestConfig = serde_yaml::from_reader(f)?;

    Ok(request_config)
}

pub fn make_request(url: &str, body: Option<String>, method: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let request_builder = match method.unwrap_or("GET".to_string()).to_uppercase().as_str() {
        "DELETE"=> Ok(client.delete(url)),
        "GET"=> Ok(client.get(url)),
        "POST"=> Ok(client.post(url)),
        "PUT"=> Ok(client.put(url)),
        "PATCH"=> Ok(client.put(url)),
        _ => Err("Unknown http method"),
    }?;

    let request_builder = match body {
        Some(body) => request_builder.body(body),
        None => request_builder,
    };

    let response = request_builder.send()?;

    debug!("{:?}", response);

    Ok(response.text()?)
}
 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
