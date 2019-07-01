
use proc_macro::TokenStream;
use crate::error::Errors;


/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Errors> {

    // TODO: generate wrapper function
//    parser::reset_attrs_used();
//    let item = syn::parse2::<syn::Item>(input)?;
//    let opts = syn::parse2(attr)?;
//
//    let mut tokens = proc_macro2::TokenStream::new();
//
//    println!("Item: {:?}", item);
//    println!("Opts {:?}", opts);

//    let mut program = backend::ast::Program::default();
//    item.macro_parse(&mut program, (Some(opts), &mut tokens))?;
//    program.try_to_tokens(&mut tokens)?;

    // If we successfully got here then we should have used up all attributes
    // and considered all of them to see if they were used. If one was forgotten
    // that's a bug on our end, so sanity check here.
//    parser::assert_all_attrs_checked();

    Ok(input)
}