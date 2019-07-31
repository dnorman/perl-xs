#[macro_use]
extern crate cstr;
#[macro_use]
extern crate perl_xs;
#[macro_use]
extern crate perl_sys;

mod stack;
mod scalar;
mod array;
mod hash;
mod panic;
mod param;
mod data;
mod derive;

//perlxs_bootstrap!(XSTest);
//pub extern "C" fn boot_XSTest (pthx: *mut perl_xs::raw::Interpreter, _cv: *mut perl_xs::raw::CV) {
pthx! {
    #[no_mangle]
    #[allow(non_snake_case)]
    fn boot_XSTest (pthx, _cv: *mut perl_xs::raw::CV) {
        println!("BOOT");
        let perl = perl_xs::raw::initialize(pthx);
        perl_xs::context::Context::wrap(perl, |ctx| {

            for (symbol, ptr) in perl_xs::REGISTRY.iter() {
                println!("BOOT - FOUND {}", symbol);
                let cname = ::std::ffi::CString::new(symbol.to_owned()).unwrap();
                ctx.new_xs(&cname, *ptr);
            }

            1 as perl_xs::raw::IV
        });
    }
}


//xs! {
//    bootstrap boot_XSTest;
//    use stack;
//    use scalar;
//    use array;
//    use hash;
//    use panic;
//    use param;
//    use data;
//    use derive;
//}
