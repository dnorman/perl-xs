
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

    let rust_name = f.ident.clone();
    let xs_name = syn::Ident::new(&format!("_xs_{}",rust_name),f.ident.span());

    let output = quote!{
        pub extern "C" fn #xs_name (pthx: *mut ::perl_sys::types::PerlInterpreter, _cv: *mut ::perl_xs::raw::CV) {

            let perl = ::perl_xs::raw::initialize(pthx);
            ::perl_xs::context::Context::wrap(perl,|mut ctx| {
                let _result = #rust_name(ctx);
            });

        }

        #f
    };

    Ok(output)
}


//                                              let mut _arg = 0;
//                                              let len =
//                                                  match ctx.st_try_fetch::<UV>(_arg)
//                                                      {
//                                                          Some(Ok(v)) =>
//                                                              v,
//                                                          Some(Err(e)) =>
//                                                              {
//                                                                  croak!("invalid argument \'len\' for GTCore::Util::RandString::randstring: ")
//                                                              }
//                                                          None => {
//                                                              croak!("not enough arguments for GTCore::Util::RandString::randstring")
//                                                          }
//                                                      };
//                                              _arg += 1;
//                                              let chars =
//                                                  match ctx.st_try_fetch::<AV>(_arg)
//                                                      {
//                                                          Some(Ok(v)) =>
//                                                              Some(v),
//                                                          Some(Err(e)) =>
//                                                              {
//                                                                  croak!("invalid argument \'chars\' for GTCore::Util::RandString::randstring: ")
//                                                              }
//                                                          None => {
//                                                              None
//                                                          }
//                                                      };
//                                              _arg += 1;
//                                              {
//                                                  let charset : Option<Vec<char>> = match chars {
//                                                      Some(c) => {
//                                                          Some( c.iter().filter_map(|sv| sv).map(|sv: SV| sv.to_string().unwrap().chars().next().unwrap()).collect() )
//                                                      },
//                                                      None => None
//                                                  };