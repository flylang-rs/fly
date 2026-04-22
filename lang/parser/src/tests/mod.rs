pub mod utils;

pub mod functions;
pub mod records;

#[test]
fn no_code() {
    let code = utils::code2ast("");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn only_spaces() {
    let code = utils::code2ast("       ");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn only_spaces_and_tabs() {
    let code = utils::code2ast(" \t\t   \t   \t\t\t\t\t\t");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn spaces_newlines_and_tabs() {
    let code = utils::code2ast("    \n\n \n\t\t     ");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn addition() {
    let code = utils::code2ast("2 + 4");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn addition_mul() {
    let code = utils::code2ast("2 + 2 * 2");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn addition_mul2() {
    let code = utils::code2ast("2 * 2 + 2");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn expr_paren() {
    let code = utils::code2ast("4 * (2 + 2) + 2");

    insta::assert_debug_snapshot!(code);
}

#[test]
fn simple_assignment() {
    let code = utils::code2ast(
        r#"
four = 2 + 2
nine = 4 + 5
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn comments() {
    let code = utils::code2ast(
        r#"
# Comment
# Comment number 2
3 + 8

# Opa!
4 + 9
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn exprs_and_calls() {
    let code = utils::code2ast(
        r#"
a = foo(1, 2, 3, 4)
b = (2 + 2) * 2
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn multiassign() {
    let code = utils::code2ast(
        r#"
# Test multiple assignment

[a, b] = [4, 9]
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn if_else() {
    let code = utils::code2ast(
        r#"
number = 49

if number == 49 {
    print("Forty-nine looks like fortune!")
} else if number < 49 {
    print("It's too early!")
} else if number > 49 {
    print("It's too late!")
} else {
    print("Intapreta guna sei wut")
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn multiassign_2() {
    let code = utils::code2ast(
        r#"
a = b = c = 1
d = a + b
e = d + a
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn modules_props_and_scopes() {
    let code = utils::code2ast(
        r#"
# Modules

use wings;

# Properties

wings.flap()

# Scopes

use(a = "Opa, I have birds too! And mine are better, 'cause they have mics!") {
    print(a)
}
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn advanced_props() {
    let code = utils::code2ast(
        r#"
ninja = Character()

ninja.first_name = "Lloyd"
ninja.middle_name = "Montgomery"
ninja.last_name = "Garmadon"

ninja.gi.color = GiColor.Green
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn property_calls() {
    let code = utils::code2ast(
        r#"
a = A()
d = a.as_b().as_c().into().unwrap()
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn simple_property_eval() {
    let code = utils::code2ast(
        r#"
a.b + c.d
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn array_indexing() {
    let code = utils::code2ast(
        r#"
a = [1, 2, 3]

b = a[0] + a[1] + a[2]
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn arrays() {
    let code = utils::code2ast(
        r#"
# Array

a = [1, 2, 3]
b = [1, 2, [3, 4, 5]]
c = [a + b]
d = a[0 + 1]
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn comparison_with_expr() {
    let code = utils::code2ast(
        r#"
2 % 4 == 0
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn private_variables() {
    let code = utils::code2ast(
        r#"
a = nil

func a() {
	private a = 7
}

private b = 6
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn error_01() {
    let code = utils::code2ast(
        r#"
a, = 8
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn error_02() {
    let code = utils::code2ast(
        r#"
abc = 3 +
    "#,
    );

    insta::assert_debug_snapshot!(code);
}

#[test]
fn error_03() {
    let code = utils::code2ast(
        r#"
= [4, 3]
    "#,
    );

    insta::assert_debug_snapshot!(code);
}
