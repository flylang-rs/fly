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

#[test]
fn empty_record() {
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

#[test]
fn error_record_with_reserved_keyword_name() {
    let code = utils::code2ast(
        r#"
record new {
    private value
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn error_record_with_reserved_keyword_method() {
    let code = utils::code2ast(
        r#"
record A {
    private value
}

func A::new() {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn error_record_object_creation_without_new_keyword() {
    let code = utils::code2ast(
        r#"
record A {
    private value
}

a = A { value: 4.9 }
    "#,
    );

    insta::assert_debug_snapshot!(code);
}


#[test]
fn record_object_creation() {
    let code = utils::code2ast(
        r#"
record A {
    private value
}

a = new A { value: 4.9 }
    "#,
    );

    insta::assert_debug_snapshot!(code);
}