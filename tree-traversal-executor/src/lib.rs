use core::time::Duration;
use std::sync::{Arc, RwLock};

use flylang_common::spanned::Spanned;
use flylang_parser::ast::{DivisionKind, ExprKind, Expression, Statement, While};
use log::debug;

use crate::{
    control_flow::ControlFlow,
    function::Function,
    object::{LValue, Value},
    realm::Realm,
};

pub mod control_flow;
pub mod function;
pub mod object;
pub mod realm;
pub mod runtime;
pub mod types;

pub type SharedRealm = Arc<RwLock<Realm>>;

/// Entry point of out executor, it accepts a list of statements gave by parser.
/// Since it accepts any kind of statement including expressions, it will return a value.
pub fn execute(ast: Vec<Statement>) -> ControlFlow {
    let mut world = Realm::new();

    // Import native functions from modules.
    // Chain 'em all!
    let natives = core::iter::empty()
        .chain(runtime::booleans::EXPORT.iter())
        .chain(runtime::floats::EXPORT.iter())
        .chain(runtime::integers::EXPORT.iter())
        .chain(runtime::print::EXPORT.iter())
        .chain(runtime::strings::EXPORT.iter());

    // Import native functions into the world.
    for (name, func) in natives {
        world
            .values_mut()
            .insert(name.to_string(), Value::Native(*func));
    }

    exec_inner(Arc::new(RwLock::new(world)), &ast)
}

/// Trampoline for executor: operate with given realm and the parsed code
fn exec_inner(realm: SharedRealm, ast: &[Statement]) -> ControlFlow {
    for i in ast {
        let stmt = exec_single_statement(Arc::clone(&realm), i);

        match stmt {
            Some(ControlFlow::Nothing) => continue,
            Some(v) => return v,
            None => continue,
        }
    }

    debug!("Nothing returned");

    ControlFlow::Nothing
}

/// Execute the single statement.
/// Not all statements are expressions so they can't return a value.
/// This is why I make it return `Option<Value>`.
///
/// `return nil` will return `Some(Value::Nil)`
fn exec_single_statement(realm: SharedRealm, statement: &Statement) -> Option<ControlFlow> {
    match statement {
        Statement::Function(function) => {
            let name = &function.name.value;

            let value = Value::Function(Arc::new(Function {
                params: function
                    .arguments
                    .iter()
                    .map(|x| x.value.as_id().map(str::to_owned).unwrap())
                    .collect(),
                body: *function.body.clone(),
                closure_realm: Arc::clone(&realm),
            }));

            realm
                .write()
                .unwrap()
                .values_mut()
                .insert(name.clone(), value);

            None
        }
        Statement::If(stmt) => {
            // if x < n { ...
            //       ^^^^^
            // Values are accessed outside the `if` body's scope, so passing `realm` is OK.
            let cond = evaluate_expression(Arc::clone(&realm), &stmt.condition, false);

            // Condition must be a value.
            let ControlFlow::Value(cond) = cond else {
                panic!("Expected condition to evaluate into a value, got {cond:?}")
            };

            // And it must be a boolean. We don't convert anything to boolean, ...
            // ... so if an integer or string is passed into condition - it's a dev fault.
            let Value::Bool(result) = cond else {
                panic!("Expected condition to return a `boolean`, got {cond:?}")
            };

            debug!("If condition result: {result:?}");

            if result {
                let Statement::Expr(block_value) = &*stmt.body else {
                    panic!("Expected a block!")
                };

                if let ExprKind::Block(bk) = &block_value.value {
                    return Some(exec_inner(Arc::clone(&realm), &bk));
                } else {
                    panic!("Expected a block!")
                }

                // ...
            } else {
                if let Some(else_body) = &stmt.else_body {
                    exec_single_statement(realm, &else_body)
                } else {
                    Some(ControlFlow::Nothing)
                }
            }
        }
        Statement::ModuleUsageDeclaration { path } => todo!(),
        Statement::Scope { held_value, body } => todo!(),
        Statement::Return { value } => {
            let value = evaluate_expression(realm, value, false);

            debug!("Return: {value:?}");

            Some(value)
        }
        Statement::Expr(expr) => {
            debug!("Evaluating: {expr:?}");

            let expr = evaluate_expression(realm, expr, false);

            debug!("Expression: {expr:?}");

            Some(expr)
        }
        Statement::While(while_loop) => {
            let While { condition, body } = while_loop;

            loop {
                // while x < n { ...
                //       ^^^^^
                // Values are accessed outside the while body's scope, so passing `realm` is OK.
                let cond = evaluate_expression(Arc::clone(&realm), condition, false);

                // Condition must be a value.
                let ControlFlow::Value(cond) = cond else {
                    panic!("Expected condition to evaluate into a value, got {cond:?}")
                };

                // And it must be a boolean. We don't convert anything to boolean, ...
                // ... so if an integer or string is passed into condition - it's a dev fault.
                let Value::Bool(result) = cond else {
                    panic!("Expected condition to return a `boolean`, got {cond:?}")
                };

                // If condition doesn't fulfill, break outta here, easy!
                if !result {
                    break;
                }

                // And it's only beginning of the circus.
                // Usually `while` loops have a Block as their body.
                // So I'll use `evaluate_expression` to execute it, realm creation will be handled automatically!

                let Statement::Expr(block_value) = &**body else {
                    panic!("Expected a block!")
                };

                if let ExprKind::Block(bk) = &block_value.value {
                    let block_result = exec_inner(Arc::clone(&realm), &bk);

                    match block_result {
                        ControlFlow::Nothing => (),
                        ControlFlow::Value(_) => (),
                        ControlFlow::Return(_) => (),
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                    }
                } else {
                    panic!("Expected a block!")
                }
            }

            Some(ControlFlow::Nothing)
        }
        Statement::Continue => Some(ControlFlow::Continue),
        Statement::Break => Some(ControlFlow::Break),
        st => todo!("Unexpected statement: {:?}", st),
    }
}

fn binary_op_helper(
    realm: SharedRealm,
    op: &str,
    lhs: &Expression,
    rhs: &Expression,
) -> Option<Value> {
    let lhs_val = evaluate_expression(Arc::clone(&realm), lhs, true);
    let rhs_val = evaluate_expression(Arc::clone(&realm), rhs, true);

    let ControlFlow::Value(lhs_val) = lhs_val else {
        panic!("A value should be returned by LHS, got: {lhs_val:?}");
    };

    let ControlFlow::Value(rhs_val) = rhs_val else {
        panic!("A value should be returned by RHS, got: {rhs_val:?}");
    };

    binary_op_helper_values(realm, op, lhs_val, rhs_val)
}

fn binary_op_helper_values(
    realm: SharedRealm,
    op: &str,
    lhs: Value,
    rhs: Value,
) -> Option<Value> {
    let l_type = types::value_to_internal_type(&lhs).unwrap();
    let r_type = types::value_to_internal_type(&rhs).unwrap();

    let method_name = format!("{l_type}::operator{op}{r_type}");

    let method = realm.read().unwrap().lookup(&method_name);

    if let Some(method) = method {
        if let ControlFlow::Return(va) = call_func(realm, method, &[lhs, rhs]) {
            return Some(va);
        } else {
            panic!("Failed to get a return value from function call.");
        }
    } else {
        panic!("Incompatible types for operation `{op}`: `{l_type}` and `{r_type}`")
    }
}

fn unary_op_helper(realm: SharedRealm, op: &str, expr: &Expression) -> Option<Value> {
    let expr_val = evaluate_expression(Arc::clone(&realm), expr, true);

    let ControlFlow::Value(expr_val) = expr_val else {
        panic!("A value should be returned, got: {expr_val:?}");
    };

    let ty = types::value_to_internal_type(&expr_val).unwrap();

    let method_name = format!("{ty}::operator{op}");

    let method = realm.read().unwrap().lookup(&method_name);

    if let Some(method) = method {
        if let ControlFlow::Return(va) = call_func(realm, method, &[expr_val]) {
            return Some(va);
        } else {
            panic!("Failed to get a return value from function call.");
        }
    } else {
        panic!("Incompatible type for operation `{op}`: `{ty}`")
    }
}

fn resolve_lvalue(realm: SharedRealm, expr: &Expression) -> LValue {
    match &expr.value {
        ExprKind::Identifier(name) => LValue::Identifier(name.clone()),

        ExprKind::IndexedAccess { origin, index } => {
            let container = evaluate_expression(Arc::clone(&realm), origin, true);
            let index = evaluate_expression(Arc::clone(&realm), index, true);

            let ControlFlow::Value(container) = container else {
                panic!("Expected value as container, got: {container:?}");
            };

            let ControlFlow::Value(index) = index else {
                panic!("Expected value as index, got: {index:?}");
            };

            LValue::Index { container, index }
        }

        ExprKind::PropertyAccess { origin, property } => {
            let object = evaluate_expression(Arc::clone(&realm), origin, true);
            let name = property.value.as_id().unwrap().to_owned();

            let ControlFlow::Value(object) = object else {
                panic!("Expected value as object, got: {object:?}");
            };

            LValue::Property { object, name }
        }

        _ => panic!("Invalid assignment target"),
    }
}

fn read_lvalue(realm: SharedRealm, target: &LValue) -> Value {
    match target {
        LValue::Identifier(name) => realm.read().unwrap().lookup(name).unwrap(),
        LValue::Index { container, index } => {
            let Value::Array(arr) = container else {
                panic!()
            };
            let Value::Integer(i) = index else { panic!() };

            arr.lock().unwrap()[*i as usize].clone()
        }
        LValue::Property { .. } => todo!("Property value read"),
    }
}

// I separated it into a function to make it compatible with OpAssignments (+=, -=, ...)
fn assign(realm: SharedRealm, target: LValue, value: Value) {
    match target {
        LValue::Identifier(name) => {
            realm.write().unwrap().values_mut().insert(name, value);
        }
        LValue::Index { container, index } => {
            let Value::Array(arr) = container else {
                panic!("Cannot index into non-array")
            };
            let Value::Integer(i) = index else {
                panic!("Array index must be integer")
            };

            arr.lock().unwrap()[i as usize] = value;
        }
        LValue::Property { object, name } => {
            todo!("Property assignment when you add objects")
        }
    }
}

fn compound_assignment_helper(
    realm: SharedRealm,
    op: &str,
    name: &Expression,
    value: &Expression,
    is_subexpression: bool,
) -> ControlFlow {
    let target = resolve_lvalue(Arc::clone(&realm), name);

    // Read current value — identifier needs a lookup, indexed needs evaluation
    let current = read_lvalue(Arc::clone(&realm), &target);
    let rhs = evaluate_expression(Arc::clone(&realm), value, true);

    let ControlFlow::Value(rhs) = rhs else {
        panic!("Expected RHS as value, got: {rhs:?}");
    };

    let result = binary_op_helper_values(Arc::clone(&realm), op, current, rhs).unwrap();

    assign(Arc::clone(&realm), target, result.clone());

    if is_subexpression {
        ControlFlow::Value(result)
    } else {
        ControlFlow::Nothing
    }
}

// I made `is_subexpression` to track are we in a root expression or not.
// It will fix a bug when expression returned value early than expected
//
// > a = 4
// ... b = 5
// ... a + b
// = 4    # WHAT? Should be 9
//
// Note: Assignment expression returns value too to make a multiple assignment feature:
// > a = b = c = d = 9
// = 9
fn evaluate_expression(
    realm: SharedRealm,
    expr: &Expression,
    is_subexpression: bool,
) -> ControlFlow {
    let Spanned { value, address } = expr;

    debug!("Eval: {expr:?}");

    match value {
        ExprKind::Add(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "+", lhs, rhs).unwrap())
        }
        ExprKind::Sub(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "-", lhs, rhs).unwrap())
        }
        ExprKind::Mul(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "*", lhs, rhs).unwrap())
        }
        ExprKind::Div(lhs, rhs, division_kind) => {
            let op = match division_kind {
                DivisionKind::Neutral => "/",
                DivisionKind::RoundingUp => "/+",
                DivisionKind::RoundingDown => "/-",
            };

            ControlFlow::Value(binary_op_helper(realm, op, lhs, rhs).unwrap())
        }
        ExprKind::Mod(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "%", lhs, rhs).unwrap())
        }
        ExprKind::And(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "&&", lhs, rhs).unwrap())
        }
        ExprKind::Or(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "||", lhs, rhs).unwrap())
        }
        ExprKind::BitAnd(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "&", lhs, rhs).unwrap())
        }
        ExprKind::BitOr(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "|", lhs, rhs).unwrap())
        }
        ExprKind::BitShiftLeft(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "<<", lhs, rhs).unwrap())
        }
        ExprKind::BitShiftRight(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, ">>", lhs, rhs).unwrap())
        }
        ExprKind::Equals(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "==", lhs, rhs).unwrap())
        }
        ExprKind::Greater(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, ">", lhs, rhs).unwrap())
        }
        ExprKind::GreaterOrEquals(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, ">=", lhs, rhs).unwrap())
        }
        ExprKind::Less(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "<", lhs, rhs).unwrap())
        }
        ExprKind::LessOrEquals(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "<=", lhs, rhs).unwrap())
        }
        ExprKind::Not(val) => ControlFlow::Value(unary_op_helper(realm, "!", val).unwrap()),
        ExprKind::Neg(val) => ControlFlow::Value(unary_op_helper(realm, "-", val).unwrap()),
        ExprKind::Identifier(id) => {
            let value = realm.read().unwrap().lookup(id.as_str());

            if value.is_none() {
                panic!("Name `{id}` is not defined.");
            }

            ControlFlow::Value(value.unwrap())
        }
        ExprKind::Number(nr) => {
            let is_float = nr.contains('.');

            let val = if is_float {
                Value::Float(nr.parse::<f64>().unwrap())
            } else {
                Value::Integer(nr.parse::<i128>().unwrap())
            };

            ControlFlow::Value(val)
        }
        ExprKind::String(st) => ControlFlow::Value(Value::String(Arc::new(st.clone()))),
        ExprKind::Block(ast) => {
            let inner_realm = Arc::new(RwLock::new(Realm::dive(Arc::clone(&realm))));
            let block_result = exec_inner(inner_realm, ast);

            match block_result {
                ControlFlow::Return(_) => block_result,
                ControlFlow::Value(v) => ControlFlow::Value(v),
                ControlFlow::Nothing => ControlFlow::Nothing,
                other => other,
            }
        }
        ExprKind::Array(spanneds) => todo!(),
        ExprKind::Call { callee, parameters } => {
            let func = evaluate_expression(Arc::clone(&realm), callee, true);

            let ControlFlow::Value(func) = func else {
                panic!("Expected a function as value, got: {func:?}");
            };

            let args: Vec<Value> = parameters
                .iter()
                .map(|x| {
                    let expr = evaluate_expression(Arc::clone(&realm), x, true);

                    if let ControlFlow::Value(va) = expr {
                        va
                    } else {
                        panic!("Expected value, got: {expr:?}");
                    }
                })
                .collect();

            debug!("Calling func with args: {:?}", args);

            call_func(realm, func, &args)
        }
        ExprKind::Assignment { name, value } => {
            // FIXME: It can only assign to plain vaiables, but can't assign by array index or object path:
            // `myvar = 4` - works
            // `myvar[0] = 4` - DOESN'T work
            // `myvar.a.b.c = 4` - DOESN'T work
            let target = resolve_lvalue(Arc::clone(&realm), name);
            let rhs = evaluate_expression(Arc::clone(&realm), value, true);

            let ControlFlow::Value(rhs) = rhs else {
                panic!("Expected RHS as value, got: {rhs:?}");
            };

            assign(Arc::clone(&realm), target, rhs.clone());

            if is_subexpression {
                ControlFlow::Value(rhs)
            } else {
                ControlFlow::Nothing
            }
        }
        ExprKind::PropertyAccess { origin, property } => todo!(),
        ExprKind::IndexedAccess { origin, index } => todo!(),

        ExprKind::AddAssign(lhs, rhs) => compound_assignment_helper(realm, "+", lhs, rhs, is_subexpression),
        ExprKind::SubAssign(lhs, rhs) => compound_assignment_helper(realm, "-", lhs, rhs, is_subexpression),
        ExprKind::MulAssign(lhs, rhs) => compound_assignment_helper(realm, "*", lhs, rhs, is_subexpression),
        ExprKind::DivAssign(lhs, rhs, division_kind) => {
            let op = match division_kind {
                DivisionKind::Neutral => "/",
                DivisionKind::RoundingUp => "/+",
                DivisionKind::RoundingDown => "/-",
            };

            compound_assignment_helper(realm, op, lhs, rhs, is_subexpression)
        },
        ExprKind::ModAssign(lhs, rhs) => compound_assignment_helper(realm, "%", lhs, rhs, is_subexpression),

        ExprKind::NotEquals(lhs, rhs) => {
            ControlFlow::Value(binary_op_helper(realm, "!=", lhs, rhs).unwrap())
        }

        ExprKind::True => ControlFlow::Value(Value::Bool(true)),
        ExprKind::False => ControlFlow::Value(Value::Bool(false)),
    }
}

// Performs a function call.
// Supported both native and regular functions.
fn call_func(realm: SharedRealm, func: Value, args: &[Value]) -> ControlFlow {
    if let Value::Native(native) = func {
        let new_realm = Realm::dive(realm);

        return native(Arc::new(RwLock::new(new_realm)), args);
    }

    if let Value::Function(func) = func {
        let mut new_realm = Realm::dive(Arc::clone(&func.closure_realm));

        let parameters = &func.params;

        if parameters.len() != args.len() {
            panic!("Insufficent arguments!");
        }

        // Arguments are just temporary variables

        for (par, arg) in parameters.iter().zip(args) {
            new_realm.values_mut().insert(par.clone(), arg.clone());
        }

        let result = exec_single_statement(Arc::new(RwLock::new(new_realm)), &func.body)
            .unwrap_or(ControlFlow::Nothing);

        debug!(
            "Executing func with params {:?} returned {:?}",
            func.params, result
        );

        return result;
    }

    ControlFlow::Nothing
}
