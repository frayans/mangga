## `mangga`
A tool to manage your manga collection.
This is still a W.I.P and I made it for my personal use,
any contributions are still welcome.

## Project structure
- `src` contains all of the source code
  * `src/main.rs` just calls the `run()` function from `src/cli`
  * `src/cli` contains the command-line interface (CLI) code.
    All of the subcommands are separated into its own file e.g.
    ```
    mangga filename
    ```
    is in 
      ```
    src/cli/filename.rs
    ```
  * `src/core` contains the business logic
    The name of the file e.g.
    ```
    src/core/filename.rs
    ```
    corresponds to the subcommand name, in this case it'd be:
    ```
    mangga filename
    ```
