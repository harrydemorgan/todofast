# todofast

My fast to do list made in rust.

## Usage
```
Usage:
todo <action> <argument>

Actions:
todo
todo add <String>
todo remove <Index>
```

## Setup

### Building from source
1. Clone the repository:
   ```
   git clone https://github.com/harrydemorgan/todofast
   ```
2. cd into directory and build with release flag
   ```
   cargo build --release
   ```
3. Move executable to PATH
   MacOS/Linux:
   ```
   sudo mv target/release/todo /usr/local/bin/
   ```
