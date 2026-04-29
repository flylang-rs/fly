use crate::tests::utils::{Tester, execute};

mod utils;

#[test]
fn simple_arithmetics() {
    let result = execute("2 + 2").unwrap();

    assert_eq!(result.as_value().unwrap().as_integer().unwrap(), 4);
}

#[test]
fn string_concat() {
    let result = execute("'Hi, ' + 'Flylang!'").unwrap();

    assert_eq!(result.as_value().unwrap().as_arc_string().unwrap().as_str(), "Hi, Flylang!");
}

#[test]
fn bool_operations() {
    let result = execute(r#"
a = true
b = false

c = [a && b, a || b, !a, !b]

c.to_string()
"#).unwrap();

	let bind = result.as_value().unwrap();
	let bind2 = bind.as_arc_string().unwrap();
	let val = bind2.as_str();

    assert_eq!(val, "[false, true, false, true]");
}

#[test]
fn global_values() {
    let code = r#"
glo = 7

func test(a, b) {
    return a + b + glo
}

test(4, 9)
    "#;

    let result = execute(code).unwrap();

    assert_eq!(result.as_value().unwrap().as_integer().unwrap(), 20);
}

#[test]
fn variables() {
    let code = r#"
a = 4
b = 9

a + b
    "#;

    let result = execute(code).unwrap();

    assert_eq!(result.as_value().unwrap().as_integer().unwrap(), 13);
}

#[test]
fn expr_evaluation_check() {
    let code = r#"
func foo() {
    4 + 9

    return 3 + 3;
}

foo()
    "#;

    let result = execute(code).unwrap();

    assert_eq!(result.as_value().unwrap().as_integer().unwrap(), 6);
}

#[test]
fn arrays_and_ref_cycles() {
    let mut tester = Tester::new();

    let result = tester.exec("[1, 2, 3, 4].to_string()");
    let string_repr = result.unwrap().as_value().unwrap().as_arc_string().unwrap();

    assert_eq!(&*string_repr, "[1, 2, 3, 4]");

    let result = tester.exec("[1, 2, [3, 4]].to_string()");
    let string_repr = result.unwrap().as_value().unwrap().as_arc_string().unwrap();

    assert_eq!(&*string_repr, "[1, 2, [3, 4]]");

    let result = tester.exec("[1, 2, [3, 4, [5, 6]]].to_string()");
    let string_repr = result.unwrap().as_value().unwrap().as_arc_string().unwrap();

    assert_eq!(&*string_repr, "[1, 2, [3, 4, [5, 6]]]");

    tester
        .exec_script(
            r#"
a = [1, 2]
b = [3, 4, a]

a.push(b)
a.push(5)
    "#,
        )
        .unwrap();

    let result = tester.exec("a.to_string()");
    let string_repr = result.unwrap().as_value().unwrap().as_arc_string().unwrap();

    assert_eq!(&*string_repr, "[1, 2, [3, 4, [...]], 5]");

    let result = tester.exec("b.to_string()");
    let string_repr = result.unwrap().as_value().unwrap().as_arc_string().unwrap();

    assert_eq!(&*string_repr, "[3, 4, [1, 2, [...], 5]]");
}
