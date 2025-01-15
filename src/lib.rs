use std::fs::read_to_string;

use proc_macro::TokenStream;
use quote::quote;


#[proc_macro]
pub fn dotenv(input: TokenStream) -> TokenStream {
    // Parse the input
    let env_variable: syn::LitStr = syn::parse(input).expect("Only pass a single string into the macro!");
    // Get contents of .env file
    let dotenv_file_contents = read_to_string(".env").expect(".env file not found");
    // Find the appropriate line, and split it on spaces
    let mut line = dotenv_file_contents.lines().filter(|line| line.contains(&env_variable.value())).next().expect(&format!("Environment variable \"{}\" not found", env_variable.value())).split("=");
    // Ignore the key and equal sign
    line.next();
    // Get the value of the env variable
    let value = line.next().expect("No value found").trim();
    // Generate it into rust code
    quote! {
        #value
    }.into()
}

#[proc_macro]
pub fn dotenv_option(input: TokenStream) -> TokenStream {
    // Parse the input
    let env_variable: syn::LitStr = syn::parse(input).expect("Only pass a single string into the macro!");
    // Get contents of .env file
    if let Ok(dotenv_file_contents) = read_to_string(".env") {
        // Find the appropriate line, and split it on spaces
        if let Some(line) = dotenv_file_contents.lines().filter(|line| line.contains(&env_variable.value())).next() {
            let mut line = line.split(" ");
            // Ignore the key and equal sign
            line.next();
            line.next();
            // Get the value of the env variable
            if let Some(value) = line.next() {
                // Generate it into rust code
                quote! {
                    Some(#value)
                }.into()
            } else {
                quote! {
                    None
                }.into()
            }
            
        } else {
            quote! {
                None
            }.into()
        }
        
    } else {
        quote! {
            None
        }.into()
    }
    
}