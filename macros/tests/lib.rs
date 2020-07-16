#[macros::nop]
fn wrapped_function() {
    unimplemented!();
}

#[test]
fn it_works() {
    assert_panic!(wrapped_function());
}
