# first program using cargo

- **cargo** is the package manager for rust
- To create a project with cargo run `cargo new my_awesome_project`, this commands create the basic structure for a cargo project, also creates te directory for our project
- Inside the project directory we have 3 files the `Cargo.toml` file,a `.gitignore` file for the VCS and a `src` directory.
- The `Cargo.toml` is the configuration file for our project, is a TOML file, it contains the required configuration for a rust project.
- The `.gitignore` file is a Git file, cargo use Git as default VCS but also we can use other.
- The `src` file is the main folder to store our project files, cargo expects all and only source code inside this folder, the root directory must be used to store README, licence and other files.
- When you run `cargo build` the generated/compiled code is stored in **target/debug** directory, this command build in debug mode, for a production/release we must use `--release` flag, this flag compile our project with optimizations(to make the Rust code run faster) and the files generated are stored in **target/release**.
- All the cargo commands are equal for all SO.

## Cargo commands

- `cargo new awesome_app`: Creates a new rust project with the given name.
- `cargo build` : It builds the current project in debug mode
- `cargo build --release` : It builds the current project in release mode,it optimize our source code.
- `cargo run`: Ir runs the current project.
- `cargo check`: This command allow check our code and make sure it our code compiles but doesn't produce an executable.
- `cargo 'command' --help`: To inspect the command and see all available options.
