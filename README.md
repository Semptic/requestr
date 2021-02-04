[![Crates.io](https://img.shields.io/crates/v/requestr-cli?label=reqestr-cli)](https://crates.io/crates/requestr-cli)
[![Crates.io](https://img.shields.io/crates/v/requestr-core?label=reqestr-core)](https://crates.io/crates/requestr-core)
[![GitHub](https://img.shields.io/github/license/Semptic/requestr)](https://github.com/Semptic/requestr/blob/main/LICENSE)
![test](https://github.com/Semptic/requestr/workflows/test/badge.svg)
[![codecov](https://codecov.io/gh/Semptic/requestr/branch/main/graph/badge.svg?token=S6EB8UJVPD)](https://codecov.io/gh/Semptic/requestr)


# requestr
Store, share and run http request templates easily


## Install

Install the cli tool with `cargo install requestr-cli`.
## Usage

### Request Configuration

requestr uses request configuration to make HTTP requests. In the 
request template you can store following:

* url
* method
* headers
* body

If you want to send a json to an endpoint you would setup following config. 
The config is stored in a yaml file.

```yaml
url: https://your.host.com/api/user
method: post
header:
  Content-Type: application/json
  Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ== 
body: |
  {
    "name": "Stefan",
    "id": 1001
  }
```

If you store this in `./request.yaml`, you can make a request with `requestr request.yaml`

### Request Config Template

To increase reusability of the request configuration you can use template variables. We can 
use `{{ variable_name }}` to insert variables into the request config.

Using the above example it would be great if we could provide the `name` and the `id` 
from the cli. The modified config would look like:

```yaml
url: https://your.host.com/api/user
method: post
header:
  Content-Type: application/json
  Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ== 
body: |
  {
    "name": "{{ user_name }}",
    "id": {{ user_id }}
  }
```

We can now provide `user_name` and `user_id` from the cli: `requestr request.yaml -p user_name=Stefan -p user_id=42`
### Environment config

For a lot of requests we want to share common template variables for different environments. One example could be different 
credentials and hosts for your test and production environment. To cope with this it is possible to create a yaml file
per environment.

We modify the above example and add variables for the host and the Authorization.

request.yaml
```yaml
url: https://{{ host }}/api/user
method: post
header:
  Content-Type: application/json
  Authorization: Basic {{ auth }}
body: |
  {
    "name": "{{ user_name }}",
    "id": {{ user_id }}
  }
```

To switch quickly between test and prod we can create following environment configs.

test.yaml
```yaml
host: test-api.com
auth: 11111111111111111111111111==
```

prod.yaml
```yaml
host: api.com
auth: 22222222222222222222222222==
```

To use one of those files you can run
`requestr request.yaml --env test.yaml -p user_name=Stefan -p user_id=42`
