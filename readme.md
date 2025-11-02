# Sarvil Shell

Sarvil Shell is a simple command-line shell written in Rust.  
It supports basic built-in commands like `echo`, `cd`, `pwd`, `cat`, `ls`, `type`, and `exit`.  
The shell also supports output redirection using the `>` operator (e.g., `echo hello > file.txt`).

---

## Features

- Basic command parsing and execution
- Built-in commands implemented in Rust
- Redirection support (`>`)
- Clear modular design (`builtins`, `external`, `std_out` modules)
- Easily extendable architecture for future commands

---

## Project Structure

src/
├── main.rs # Entry point for the shell
├── shell/
│ ├── mod.rs # CommandType enum and command parsing logic
│ ├── builtins.rs # Implementation of built-in commands
│ ├── external.rs # Handling of external commands
│ └── std_out.rs # Output redirection (>)



---

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/ken-kaneki-9/sarvil_shell.git
   cd sarvil_shell
   cargo run
 
## Example Commands
echo Hello World
pwd
type echo
echo Hello > output.txt
exit 0


## To-Do

>Add support for input redirection (<)[I am working on this]
>Add piping (|) between commands
> Implement history and autocomplete
> Improve error handling and logging
> Add support for running background jobs

##Contribution 

Contributions are welcome.
To contribute:

Fork the repository

Create a new branch for your feature or bugfix:

git checkout -b feature-name


Make your changes and test them

Commit your changes with a clear message:

git commit -m "Add description of change"


Push your branch and create a pull request:

git push origin feature-name


Please make sure your code is formatted using cargo fmt and passes basic checks with cargo check.
