# Rust PetClinic Sample Application

A sample Rust application modeled after [Spring PetClinic](https://github.com/spring-projects/spring-petclinic).

## Run the application
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