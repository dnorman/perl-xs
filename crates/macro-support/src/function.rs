
use proc_macro2::TokenStream;
use crate::error::Errors;
use quote::quote;

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Errors> {

    // TODO: generate wrapper function
//    parser::reset_attrs_used();
    let item = syn::parse2::<syn::Item>(input.clone())?;
//    let opts = syn::parse2(attr)?;

   match item {
       syn::Item::Fn(f) => {
           expand_function(f)
       },
       _ => panic!("cannot expand macro for non-function")
   }
//    let mut tokens = proc_macro2::TokenStream::new();
}

fn expand_function (f: syn::ItemFn ) -> Result<TokenStream,Errors>{
//    println!("Item: {:?}", item);
//    println!("Opts {:?}", opts);

//    let mut program = backend::ast::Program::default();
//    item.macro_parse(&mut program, (Some(opts), &mut tokens))?;
//    program.try_to_tokens(&mut tokens)?;

    // If we successfully got here then we should have used up all attributes
    // and considered all of them to see if they were used. If one was forgotten
    // that's a bug on our end, so sanity check here.
//    parser::assert_all_attrs_checked();

    let rust_fn_ident = f.ident.clone();
    let rust_fn_name = format!("{}",f.ident);
    let perl_fn_name = format!("Test::Foo::{}", rust_fn_name);

    let xs_name = syn::Ident::new(&format!("_xs_{}",rust_fn_name),f.ident.span());

    let (impl_generics, ty_generics, where_clause) = f.decl.generics.split_for_impl();

    let errors = crate::error::Errors::new();

    let mut rust_arg_unpacks = Vec::new();
    let mut rust_args = Vec::new();

    for arg in f.decl.inputs.iter(){
        println!("{:?}", arg);
        match arg {
            syn::FnArg::SelfRef(_) => {
                //TODO: determine how to implement a proxy struct for perl objects
                //      Does it entail automatic implementation of a Context trait + automatic struct instantiation?
                unimplemented!()
            },
            syn::FnArg::SelfValue(_) => {
                //TODO: determine if this is appropriate to implement
                unimplemented!()
            }
            syn::FnArg::Captured(c) => {
                if let syn::Pat::Ident(syn::PatIdent{ ident: ref arg_ident , .. }) = c.pat {
                    let var = syn::Ident::new(&format!("value_{}", arg_ident), proc_macro2::Span::call_site());

                    let rust_arg_name = format!("{}", arg_ident);
                    let rust_arg_type = &c.ty;

                    let fetch = quote! {
                        let #var = match <#rust_arg_type as TryFromContext>::try_from_context(_xs_ctx, #rust_arg_name, &mut offset){
                              Ok(v)  => v,
                              Err(e) => croak!(format!("{} for {}",e, #perl_fn_name)),
                        }
                    };

                    rust_arg_unpacks.push(fetch );
                    rust_args.push( var );
                }

            },
            syn::FnArg::Inferred(_) => {
                unimplemented!()
            },
            syn::FnArg::Ignored(_) => {

            }
        }
    }

    errors.check().unwrap();

    let output = quote!{

        #f

        pub extern "C" fn #xs_name (pthx: *mut ::perl_sys::types::PerlInterpreter, _cv: *mut ::perl_xs::raw::CV) {

            let perl = ::perl_xs::raw::initialize(pthx);
            ::perl_xs::context::Context::wrap(perl,|mut _xs_ctx| {

                let mut offset : isize = 0;
                #(#rust_arg_unpacks;)*

                #rust_fn_ident(#(#rust_args,)*)

            });

        }
    };

    Ok(output)
}