# Learning Rust by building mini projects

## Project 1: GRRS

Simple grep.

* Directory: projects/grrs
* Based on https://rust-cli.github.io/book/tutorial/index.html

**Usage docs:** 

From the project directory, run `cargo run -- --help` to see how to use.

**Example:**

```sh
projects/grrs$ cargo run hello fixtures/test-file.txt --case-sensitive --lines
```

**Libraries used:**

- [clap](https://docs.rs/clap/latest/clap/): For CLI
- [log](https://docs.rs/log/latest/log/): For logging
- [env_logger](https://docs.rs/env_logger/latest/env_logger/): Using environmental variables to set logging levels


**Testing:**

From the project directory, run `cargo test`

