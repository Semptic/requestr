use log::debug;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    collections::{HashMap, HashSet},
    error, fmt, fs,
};
use template::Template;

mod template;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestConfig {
    pub url: String,
    pub body: Option<String>,
    pub method: Option<String>,
}

pub fn load_request_template(filename: &str) -> Result<Template, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;

    let request_config_template = Template::new(contents.as_str());
    debug!("{:?}", request_config_template);

    Ok(request_config_template)
}

#[derive(Debug, Clone)]
pub struct InvalidParameter {
    pub reason: String,
}

impl InvalidParameter {
    fn new(msg: &str) -> InvalidParameter {
        InvalidParameter {
            reason: msg.to_string(),
        }
    }
}

impl fmt::Display for InvalidParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl error::Error for InvalidParameter {}

pub fn validate_parameter(
    template: &Template,
    parameter: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("{:?}", parameter);

    let provided_names: HashSet<_> = parameter.keys().cloned().collect();
    let names: HashSet<_> = template.names.iter().cloned().collect();

    let from_input: HashSet<_> = provided_names.difference(&names).collect();
    let from_template: HashSet<_> = names.difference(&provided_names).collect();

    if (from_input.len() > 0) {
        debug!(
            "Following parameters are defined but not used: {:?}",
            from_input
        );
    }

    if (from_template.len() > 0) {
        Err(InvalidParameter::new(
            &format!("Following parameters are missing: {:?}", from_template).to_string(),
        )
        .into())
    } else {
        Ok(())
    }
}

pub fn load_request_definition(
    template: &Template,
    parameter: &HashMap<String, String>,
) -> Result<RequestConfig, Box<dyn std::error::Error>> {
    let request_config_string = template.render(parameter);

    let request_config: RequestConfig = serde_yaml::from_str(request_config_string.as_str())?;

    debug!("{:?}", request_config);

    Ok(request_config)
}

pub fn make_request(
    url: &str,
    body: Option<String>,
    method: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let request_builder = match method.unwrap_or("GET".to_string()).to_uppercase().as_str() {
        "DELETE" => Ok(client.delete(url)),
        "GET" => Ok(client.get(url)),
        "POST" => Ok(client.post(url)),
        "PUT" => Ok(client.put(url)),
        "PATCH" => Ok(client.put(url)),
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
