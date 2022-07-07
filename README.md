# Rust PetClinic Sample Application

A sample Rust application modeled after [Spring PetClinic](https://github.com/spring-projects/spring-petclinic).

## Run the server application
```
cd server
```

Start the application by running:
```
cargo run
```
The application will start on port 3000.

You can then access the `/vets` endpoint:
```
curl -v localhost:3000/vets
```

Test the application by running:
```
cargo test
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