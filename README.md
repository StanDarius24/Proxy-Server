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
receiver:
  url: /test
  headers:
    Accept: application/json
    Content-Type: application/json
  query_params:
    query1: test
    query2: test
  authorization:
    basic: test
  body:
sender:
  url: /test2
  headers:
    Accept: application/json
    Content-Type: application/json
  query_params:
    query1: test
    query2: test
  authorization:
    basic: test
  body:
```

Requests sent to http://localhost:8080/test will be forwarded to http://localhost:8080/test2. (in this particular example)

For testing, use the Docker image in the interceptor directory: send a request to the app, which then calls the interceptor and receives the response.