use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use thiserror::Error;

mod template;

pub use template::Template;

#[derive(Error, Debug)]
pub enum RequestrError {
    #[error("Following parameter are missing from the input: {0:#?}")]
    MissingParameter(Vec<String>),
    #[error("Unable to load Template from disk")]
    OpeningTemplateFailed(#[from] io::Error),
    #[error("Parsing template failed")]
    TemplateParsingFailed(#[from] serde_yaml::Error),
    #[error("Wrong request config: {0}")]
    BrokenRequestConfig(String),
    #[error("Request failed")]
    UnknownRequestError(#[from] reqwest::Error),
}

pub type ResultT<T> = Result<T, RequestrError>;

fn default_header() -> HashMap<String, String> {
    HashMap::new()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestConfig {
    pub url: String,
    #[serde(default = "default_header")]
    pub header: HashMap<String, String>,
    pub body: Option<String>,
    pub method: Option<String>,
}

pub fn load_request_template(filename: &PathBuf) -> ResultT<Template> {
    let contents = fs::read_to_string(filename)?;

    let request_config_template = Template::new(contents.as_str());
    debug!("{:#?}", request_config_template);

    Ok(request_config_template)
}

pub fn validate_parameter(template: &Template, parameter: &HashMap<String, String>) -> ResultT<()> {
    debug!("{:#?}", parameter);

    let provided_names: HashSet<_> = parameter.keys().cloned().collect();
    let names: HashSet<_> = template.names.iter().cloned().collect();

    let from_input: HashSet<_> = provided_names.difference(&names).collect();
    let from_template: HashSet<_> = names.difference(&provided_names).collect();

    if from_input.len() > 0 {
        info!(
            "Following parameters are defined but not used: {:?}",
            from_input
        );
    }

    if from_template.len() > 0 {
        Err(
            RequestrError::MissingParameter(from_template.into_iter().map(|p| p.clone()).collect())
                .into(),
        )
    } else {
        Ok(())
    }
}

pub fn load_request_definition(
    template: &Template,
    parameter: &HashMap<String, String>,
) -> ResultT<RequestConfig> {
    let request_config_string = template.render(parameter);

    let request_config: RequestConfig = serde_yaml::from_str(request_config_string.as_str())?;

    debug!("{:#?}", request_config);

    Ok(request_config)
}

pub fn make_request(
    url: &str,
    body: Option<String>,
    method: Option<String>,
    header: HashMap<String, String>,
) -> ResultT<String> {
    let client = reqwest::blocking::Client::new();

    let request_builder = match method.unwrap_or("GET".to_string()).to_uppercase().as_str() {
        "DELETE" => Ok(client.delete(url)),
        "GET" => Ok(client.get(url)),
        "POST" => Ok(client.post(url)),
        "PUT" => Ok(client.put(url)),
        "PATCH" => Ok(client.put(url)),
        method => Err(RequestrError::BrokenRequestConfig(format!(
            "Unknown http method: {}",
            method
        ))),
    }?;

    let request_builder = match body {
        Some(body) => request_builder.body(body),
        None => request_builder,
    };

    let request_builder = header
        .into_iter()
        .fold(request_builder, |request_builder, (name, value)| {
            request_builder.header(name.as_str(), value.as_str())
        });

    let response = request_builder.send()?;

    debug!("{:#?}", response);

    let response_body = response.text()?;

    Ok(response_body)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
