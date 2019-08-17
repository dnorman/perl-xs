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
          //  if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
          //  }
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
            use std::collections::HashMap;

            let perl = perl_xs::raw::initialize(pthx);
            perl_xs::context::Context::wrap(perl, |ctx| {

                let mut package_rewrites : HashMap<&'static str, &'static str> = HashMap::new();
                for package in perl_xs::PACKAGE_REGISTRY.iter() {
                    package_rewrites.insert(package.module, package.package);
                }

                for symbol in ::perl_xs::SYMBOL_REGISTRY.iter() {
//                    println!("BOOT - FOUND {:?}", symbol);

                    let mut symbol_name : String = symbol.module.to_string();

                    if let Some(package_rewrite) = package_rewrites.get(&symbol.module) {
                        symbol_name = package_rewrite.to_string();

                    }else{
                        let mut module_name : &str = &symbol.module;
                        let mut non_aliased_parts : Vec<&str> = Vec::new();

                        loop {
                            let mut parts = module_name.rsplitn(2,"::");
                            if let (Some(spill),Some(module_name_part)) = (parts.next(),parts.next()) {
                                non_aliased_parts.push(spill);

                                if let Some(package_rewrite) = package_rewrites.get(module_name_part) {
                                    symbol_name = package_rewrite.to_string();
                                    symbol_name.push_str("::");
                                    symbol_name.push_str(&non_aliased_parts.join("::"));
                                    break;
                                }
                                module_name = module_name_part;
                            }else{
                                break;
                            }
                        }
                    }

                    symbol_name.push_str("::");
                    symbol_name.push_str(symbol.name);

//                    println!("SYMBOL NAME: {}", symbol_name);
//
                    let cname = ::std::ffi::CString::new(symbol_name).unwrap();
                    ctx.new_xs(&cname, symbol.ptr);
                }

                1 as perl_xs::raw::IV
            });
        }
    };

    body.into()
}
