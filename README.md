# Dotenv

Small, proc-macro crate that bakes environment variables from a .env file into the compiled binary.

## Usage

`.env` file

    FOO=BAR

In rust:
```rust
const FOO = dotenv!("FOO");
```

`dotenv!("FOO")` will expand at compile time to a `&'static str` of whatever `FOO` is set to in the `.env` file (in this case "bar").