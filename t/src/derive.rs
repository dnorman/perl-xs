use perl_xs::{Context,TryFromContext,DeriveTryFromContext};
use std::io::Error;

#[perlxs]
fn test_from_kv(test: TestStruct) -> String {
    // Offset should be made automatic after arg unpacking
    format!("{:?}",test)
}

#[perlxs(package="XSTest::Derive")]
fn test_from_kv_bool(test: TestStruct) -> bool {
    true
}

#[perlxs]
fn test_from_kv_error(ctx: &mut Context) -> String {
    let mut index: isize = 0;
    let err = TestStruct::try_from_context(ctx, "thingy", &mut index).unwrap_err();
    format!("{:?}",err)
}

#[perlxs]
fn test_from_kv_error_display(ctx: &mut Context) -> String {

    let mut index : isize = 0;
    let err = TestStruct::try_from_context(ctx, "thingy", &mut index).unwrap_err();
    format!("{}",err)
}

#[perlxs]
fn test_from_kv_debug(ctx: &mut Context) -> String {

    let mut index : isize = 0;
    // Offset should be made automatic after arg unpacking
    match TestStruct::try_from_context(ctx, "thingy",&mut index) {
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
    &[("XSTest::Derive::test_from_kv", _xs_test_from_kv as ::perl_xs::raw::XSUBADDR_t),
    ("XSTest::Derive::test_from_kv_bool", _xs_test_from_kv_bool as ::perl_xs::raw::XSUBADDR_t),
    ("XSTest::Derive::test_from_kv_error", _xs_test_from_kv_bool as ::perl_xs::raw::XSUBADDR_t),
    ("XSTest::Derive::test_from_kv_error_display", _xs_test_from_kv_bool as ::perl_xs::raw::XSUBADDR_t),
    ("XSTest::Derive::test_from_kv_debug", _xs_test_from_kv_debug as ::perl_xs::raw::XSUBADDR_t),
    ];

#[derive(DeriveTryFromContext,Debug)]
struct TestStruct {
    alpha:          bool,
    beta:           String,
    #[perlxs(key="-charlie", key="-charles", key="-chuck")]
    charlie:        String,
    delta:          Option<bool>,
    #[perlxs(key = "_echo")]
    echo:          Option<String>,
}