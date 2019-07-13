extern crate proc_macro;

use proc_macro::TokenStream;
use perl_xs_macro_support as support;


#[proc_macro_attribute]
pub fn perlxs(attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("ATTR: {}", attr);
    println!("INPUT: {}", input);

    match support::function::expand(attr.into(), input.into()) {
        Ok(tokens) => {
          //  if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
          //  }
            tokens.into()
        }
        Err(_error) => panic!("Unknown expansion error"),
    }
}