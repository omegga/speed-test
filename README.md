speed-test

# Prerequisites

1. Make sure you have at least 10mb of available free space on disk
2. If not already installed on your machine, follow the instructions to install Rust at: https://www.rust-lang.org/tools/install
3. Clone this repository and ```cd``` into it

# Usage

You can test this binary using one of these 3 ways:

## Running speed-test

```$ cargo run```  

## Build an executable

```$ cargo build --release```  
```$ ./target/release/speed-test```  

## Install the binary

```$ cargo install --path .```  
```$ speed-test```  
