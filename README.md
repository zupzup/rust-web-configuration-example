# rust-web-configuration-example

An example of configuration management in a Rust web application

Run with different environments:

```bash
cargo run

RUN_ENV=Development cargo run

RUN_ENV=Testing cargo run

RUN_ENV=Production cargo run
```

Explicitly set config values:

```bash

RUN_ENV=Development EA_SERVER__PORT=9090 cargo run
```

Then, you can use the following two URLs to check some config values, passed by global variable, or passed into the handler:

```bash
curl http://localhost:8080/global
Running with interval: 100000 and rules: [Rule { name: "all", rule_set: [">5", "<100"] }, Rule { name: "none", rule_set: [] }]

curl http://localhost:8080/local
Running on port: 8080 with url: http://localhost:8080
```
