use perl_xs::{Context,FromPerlKV};
use std::io::Error;

//#[perlxs]
//fn test_from_kv(test: TestStruct) -> String {
//    // Offset should be made automatic after arg unpacking
//    format!("{:?}",test)
//}
//
//#[perlxs(package="XSTest::Derive")]
//fn test_from_kv_bool(test: TestStruct) -> bool {
//    // Offset should be made automatic after arg unpacking
//    true
//}

#[perlxs]
fn test_from_kv_debug(ctx: &mut Context) -> String {
    // Offset should be made automatic after arg unpacking
    match TestStruct::from_perl_kv(ctx, 0) {
        Ok(s) => {
            format!("{:?}",s)
        },
        Err(e) => {
            croak!(format!("{}",e));
        }
    }
}

pub const PERL_XS:
&'static [(&'static str, ::perl_xs::raw::XSUBADDR_t)] =
    &[("XSTest::Derive::test_from_kv_debug", _xs_test_from_kv_debug as ::perl_xs::raw::XSUBADDR_t),
//        ("GTCore::Util::RandString::rand_decimal", rand_decimal as ::perl_xs::raw::XSUBADDR_t)
    ];


//xs! {
//    package XSTest::Derive;
//    sub test_from_kv_debug(ctx) {
//        // Offset should be made automatic after arg unpacking
//        match TestStruct::from_perl_kv(&mut ctx, 0) {
//            Ok(s) => {
//                format!("{:?}",s)
//            },
//            Err(e) => {
//                croak!(format!("{}",e));
//            }
//        }
//    }
//
//    sub test_from_kv_error(ctx) {
//        let err = TestStruct::from_perl_kv(&mut ctx, 0).unwrap_err();
//        format!("{:?}",err)
//    }
//    sub test_from_kv_error_display(ctx) {
//        let err = TestStruct::from_perl_kv(&mut ctx, 0).unwrap_err();
//        format!("{}",err)
//    }
//}

#[derive(FromPerlKV,Debug)]
struct TestStruct {
    alpha:          bool,
    beta:           String,
    #[perlxs(key="-charlie", key="-charles", key="-chuck")]
    charlie:        String,
    delta:          Option<bool>,
    #[perlxs(key = "_echo")]
    echo:          Option<String>,
}