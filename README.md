## Proxy-Server with caching capability 

Project Board: https://github.com/users/StanDarius24/projects/1

This is a proof of concept.
==========================

The application serves as a middleware service capable of receiving HTTP requests, decoding them, 
and performing various actions, such as invoking other services with different types of authorization. 
It acts as a 'man in the middle,' providing an additional layer of security by preventing the direct 
injection of secrets into microservices. The service is designed to operate within the intranet 
as a 'hidden service' within the infrastructure.
Additionally, the application implements a caching mechanism with two layers, utilizing redis (l1) and mongoDB (l2) 
to enhance performance and efficiency.

### How to Use

Run the application with:
`
cargo build && cargo run
`

You can customize settings in `config.yml`:
```yaml
entities:
  - name: entity1
    incoming_request:
      method: GET
      server_host: 0.0.0.0:8080
      endpoint_path: /test/data/*
      headers:
        Accept: application/json
        Content-Type: application/json
      query_params:
        query1: test
        query2: test
      auth:
        basic: test
      payload: ""
    outgoing_request:
      server_host: 0.0.0.0:80
      method: GET
      endpoint_path: /test2
      headers:
        Accept: application/json
        Content-Type: application/json
      query_params:
        query1: test
        query2: test
      auth:
        basic: test
      payload: ""
```

Requests sent to http://localhost:8080/test will be forwarded to http://localhost:80/test2. (in this particular example)

For testing, use the Docker image in the interceptor directory: send a request to the app, which then calls the interceptor and receives the response.

### Wildcard *

if the incoming_request endpoint_path contains a wildcard *, it will match any path after the specified path.
For example, /test/data/** will match /test/data/123, /test/data/abc/def, etc.