extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, 
    ItemFn
};

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let block = &input.block;

    let gen = quote! {
        fn #name() {
            use std::env;
            use std::fs;
            use std::path::PathBuf;

            // Attempt to read the banner content
            let banner_content = (|| -> Result<String, Box<dyn std::error::Error>> {
                let current_dir = env::current_dir()?;
                let dir = current_dir.to_str().ok_or("Failed to convert directory path")?.to_string();
                let file_path = PathBuf::from(&dir).join("banner.txt");
                let file_nm = file_path.to_str().ok_or("Failed to convert file path")?;
                let contents = fs::read_to_string(file_nm)?;
                Ok(contents)
            })();

            // Match on the result of reading the banner content
            match banner_content {
                Ok(contents) => println!("{}", contents), // Print the banner if successful
                Err(e) => eprintln!("Failed to load banner: {}", e), // Print error message if failed
            }

            // Execute the original function body
            #block
        }
    };
    gen.into()
}