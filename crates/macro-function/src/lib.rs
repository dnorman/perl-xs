#![recursion_limit="256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use perl_xs_macro_support as support;
use proc_macro2::Span;
use quote::quote;


#[proc_macro_attribute]
pub fn perlxs(attr: TokenStream, input: TokenStream) -> TokenStream {
//    println!("ATTR: {}", attr);
//    println!("INPUT: {}", input);

    match support::function::expand(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(_error) => panic!("Unknown expansion error"),
    }
}


#[proc_macro]
pub fn package(input: TokenStream) -> TokenStream {

//    println!("INPUT: {:?}", input);

    let item = syn::parse2::<syn::Lit>(input.into()).unwrap();

    let package_name = match item {
        syn::Lit::Str(s) => s,
        _ => panic!("cannot expand macro for non-function")
    };

    let package_name_clean = package_name.value().replace("::","__");
    let boot_fn_name = syn::Ident::new(&format!("boot_{}",package_name_clean),Span::call_site());

    let body = quote! {
        const _XS_PACKAGE_DEF: () = {
            #[ctor]
            fn package_def() {
//                println!("PACKAGE DEF: {}: {}", module_path!(), #package_name);
                ::perl_xs::PACKAGE_REGISTRY.submit(::perl_xs::Package{ module: module_path!(), package: #package_name});
            }
        };

        #[no_mangle]
        #[allow(non_snake_case)]
        // TODO concat this ident
        extern "C" fn #boot_fn_name (pthx: *mut ::perl_sys::types::PerlInterpreter, _cv: *mut ::perl_xs::raw::CV) {
//            println!("BOOT");

            let perl = perl_xs::raw::initialize(pthx);
            perl_xs::context::Context::wrap(perl, |ctx| {

                ::perl_xs::boot::boot(ctx, #package_name);

                1 as perl_xs::raw::IV
            });
        }
    };

    body.into()
}
