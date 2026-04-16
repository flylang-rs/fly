use crate::{control_flow::ControlFlow, object::Value, tests::utils::{TestResult, execute, execute_or_fail}};

mod utils;

#[test]
fn simple_arithmetics() {
    let result = execute("2 + 2").unwrap();

    assert_eq!(result.as_value().unwrap().as_integer().unwrap(), 4);
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
    let code = r#"
print([1, 2, 3, 4])
print([1, 2, [3, 4]])
print([1, 2, [3, 4, [5, 6]]])

print("")

a = [1, 2]
b = [3, 4, a]

a.push(b)
a.push(5)

print(a)
print(b)
    "#;

    // let result = execute(code).unwrap();

    // assert_eq!(result.as_value().unwrap().as_integer().unwrap(), 6);

    todo!("Capture println output somehow");
}

