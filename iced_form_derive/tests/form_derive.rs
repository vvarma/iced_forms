#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-form-derive.rs");
    t.pass("tests/02-nested-and-enum.rs");
    t.pass("tests/03-handling-mods.rs");
}
