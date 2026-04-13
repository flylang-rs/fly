use super::utils;

#[test]
fn record_with_method() {
    let code = utils::code2ast(
        r#"
record Test {}

func Test::say_hi() {
	print("Hi")
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]                                                                                              fn empty_record() {
    let code = utils::code2ast(
        r#"
record Test {}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn record_with_one_pub_field() {
    let code = utils::code2ast(
        r#"
record Test { public x }
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn record_with_multiple_fields() {
    let code = utils::code2ast(
        r#"
record Test {
    public x
    public y
    public z
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn record_with_mixed_fields() {
    let code = utils::code2ast(
        r#"
record Test {
    private x
    public y
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn record_with_private_fields() {
    let code = utils::code2ast(
        r#"
record Test {
    private x
    private y
    private z
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}
