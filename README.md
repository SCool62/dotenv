# Dotenv

Small, proc-macro crate that bakes environment variables from a .env file into the compiled binary. The `dotenv!()` macro is used the same as the built-in `env!()` macro, and the `dotenv_option!()` macro is used the same as `option_env!()`.

## Usage

`.env` file

    FOO=BAR

In rust:
```rust
const FOO = dotenv!("FOO");
```

`dotenv!("FOO")` will expand at compile time to a `&'static str` of whatever `FOO` is set to in the `.env` file (in this case "bar").