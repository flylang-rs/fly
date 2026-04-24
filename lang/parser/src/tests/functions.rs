use crate::tests::utils;

#[test]
fn predefined_values_in_function_args() {
    let code = utils::code2ast(
        r#"
func log(x, base = E) {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn funcargs() {
    let code = utils::code2ast(
        r#"
func multiarg(a, b, c) {
    return a + b + c
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn hello_world() {
    let code = utils::code2ast(
        r#"
func main() {
    print("Hello, world!");

    return 0;
}

main()
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn func_with_blocks() {
    let code = utils::code2ast(
        r#"
func fly() {
    {
        {
            4 + 9
        }
    }
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn function_with_reserved_keyword() {
    let code = utils::code2ast(
        r#"
func new() {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn function_static() {
    let code = utils::code2ast(
        r#"
static func a() {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn function_private_static() {
    let code = utils::code2ast(
        r#"
private static func a() {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn error_function_static_private() {
    let code = utils::code2ast(
        r#"
# `private static` not `static private`
static private func a() {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn function_private() {
    let code = utils::code2ast(
        r#"
private func a() {
    # ...
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}
