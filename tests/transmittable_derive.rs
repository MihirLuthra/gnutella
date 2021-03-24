#[test]
fn test_transmittable_derive() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/trybuild_transmittable_derive/fields_transmittable_impl.rs");
    t.pass("tests/trybuild_transmittable_derive/valid_transmittable_derive.rs");
}
