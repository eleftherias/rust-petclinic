# Rust PetClinic Sample Application

A sample Rust application modeled after [Spring PetClinic](https://github.com/spring-projects/spring-petclinic).

## Overview

This application uses [axum](https://github.com/tokio-rs/axum) and [SeaORM](https://github.com/SeaQL/sea-orm) on the server side and [Sycamore](https://github.com/sycamore-rs/sycamore) on the client side.

There is also a `dto` library for sharing the DTOs across the server and client side. This is possible since both are written in Rust.

## Run the server application
```
cd server
```

Start the application by running:
```
cargo run
```
The application will start on port 3000.

You can then access the `/owners` endpoint:
```
curl -v localhost:3000/owners
```

Test the application by running:
```
cargo test
```

## JWT authentication
The `/vets` endpoint requires JWT authentication.

Retrieve a token from the `/token` endpoint:

```
export TOKEN=`curl -XPOST localhost:3000/token -H "Content-Type: application/json" -d '{"user":"foo","password":"bar"}'`
```
note: any user/password are accepted

Use the bearer token to access the `/vets/` endpoint:

```
curl -H "Authorization: Bearer $TOKEN" localhost:3000/vets
```

## Run the client application
```
cd client
```

Serve the application by running:
```
trunk serve
```
The application will start on port 8080.

You can view the running application by navigating to localhost:8080 in your browser.

## Feedback

I am new to Rust. Feedback on best practices is appreciated.
