use crate::test_utils;

#[test]
fn test_strings() {
    let code = r#"
"string"
"string with whitespaces"
"123456789 123456789"
"string with \n newline"
"#;

    let tokens = test_utils::code_to_tokens(code);

    insta::assert_debug_snapshot!(tokens.map(|x| x.into_values_with_positions()));
}

#[test]
fn test_identifiers() {
    let code = r#"
identifier
_prefixed_identifier
______what_about_that
_12345678
_
"#;

    let tokens = test_utils::code_to_tokens(code);

    insta::assert_debug_snapshot!(tokens.map(|x| x.into_values_with_positions()));
}

#[test]
fn test_operators() {
    let code = r#"
myvar = "abc"

la = myvar / "t"
lb = myvar /- "t"
lc = myvar /+ "t"
myvar /= "a"
myvar /-= "c"
myvar /+= "b"

a == "a"
b != "b"
c <= "c"
d >= "d"
e > "e"
f < "f"

a % b

5 % 2 == 1
"#;

    let tokens = test_utils::code_to_tokens(code);

    insta::assert_debug_snapshot!(tokens.map(|x| x.into_values_with_positions()));
}

#[test]
fn test_numbers() {
    let code = r#"
12345678
-12345678
+12345678
1234_5678

0x1234_5678_90ab_cdef
0b1010_1010
0o12345670
"#;

    let tokens = test_utils::code_to_tokens(code);

    insta::assert_debug_snapshot!(tokens.map(|x| x.into_values_with_positions()));
}

#[test]
fn test_hello_world() {
    let code = r#"
func main() {
    print("Hello, world!");
}
    "#;

    let tokens = test_utils::code_to_tokens(code);

    insta::assert_debug_snapshot!(tokens.map(|x| x.into_values_with_positions()));
}

#[test]
fn test_comments() {
    let code = r#"
# This is a comment.
this = code

# There are no multiline comments
# Duuuuuuude!
# A set of singleline comments are already one multiline comment!
some_code = again
"#;

    let tokens = test_utils::code_to_tokens(code);

    insta::assert_debug_snapshot!(tokens.map(|x| x.into_values_with_positions()));
}
