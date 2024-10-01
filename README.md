# Boot-Banner

Boot-Banner is a Rust library that provides a simple way to display a banner when your application starts. It uses a procedural macro to inject banner-displaying code into your main function.

## Features

- Easy to use: Just add a single attribute to your main function
- Customizable: Use your own banner text file
- Error-tolerant: Continues execution even if the banner file is not found

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
Boot-Banner = "0.1.0"
```

## Usage
Create a banner.txt file in your project's root directory with your desired banner content.
In your main.rs file, use the #[init] attribute on your main function:

```Rust
use Boot_Banner::init;

#[init]
fn main() {
    // Your code here
    println!("Hello, world!");
}
```

## How it works
The init attribute macro does the following:
- Attempts to read the banner.txt file from the current directory
- If successful, prints the banner content to the console
- If unsuccessful (e.g., file not found), prints an error message
- Executes the original function body
## Error Handling
If the banner file cannot be read or doesn't exist, the library will print an error message but allow your application to continue running.
## License
This project is licensed under the MIT License - see the LICENSE file for details.
## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
## Authors
m1n67un
## Acknowledgments
Thanks to the Rust community for providing excellent documentation and resources.
