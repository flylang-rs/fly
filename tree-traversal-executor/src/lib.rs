use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use flylang_common::spanned::Spanned;
use flylang_parser::ast::{DivisionKind, ExprKind, Expression, Statement};

use crate::{function::Function, object::Value, realm::Realm};

pub mod function;
pub mod object;
pub mod realm;
pub mod runtime;
pub mod types;

pub type SharedRealm = Arc<RwLock<Realm>>;

pub fn execute(ast: Vec<Statement>) -> Value {
    let mut world = Realm::new();

    let natives = core::iter::empty()
        .chain(runtime::floats::EXPORT.iter())
        .chain(runtime::integers::EXPORT.iter())
        .chain(runtime::print::EXPORT.iter())
        .chain(runtime::strings::EXPORT.iter());

    // Import native functions into the world.
    for (name, func) in natives {
        world.values_mut().insert(
            name.to_string(),
            Value::Native(*func),
        );    
    }

    exec_inner(Arc::new(RwLock::new(world)), &ast)
}

fn exec_inner(realm: SharedRealm, ast: &[Statement]) -> Value {
    for i in ast {
        if let Some(val) = exec_single_statement(Arc::clone(&realm), i) {
            return val;
        }
    }

    Value::Nil
}

fn exec_single_statement(realm: SharedRealm, statement: &Statement) -> Option<Value> {
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
        Statement::If(_) => todo!(),
        Statement::ModuleUsageDeclaration { path } => todo!(),
        Statement::Scope { held_value, body } => todo!(),
        Statement::Return { value } => {
            return evaluate_expression(realm, value, false);
        }
        Statement::Expr(expr) => evaluate_expression(realm, expr, false),
        st => todo!("Unexpected statement: {:?}", st)
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
) -> Option<Value> {
    let Spanned { value, address } = expr;

    match value {
        ExprKind::Add(lhs, rhs) => {
            let lhs = evaluate_expression(Arc::clone(&realm), lhs, true)
                .expect("Cannot be evaluated (lhs)");
            let rhs = evaluate_expression(Arc::clone(&realm), rhs, true)
                .expect("Cannot be evaluated (rhs)");

            let l_type = types::value_to_internal_type(&lhs).unwrap();
            let r_type = types::value_to_internal_type(&rhs).unwrap();

            let method_name = format!("{l_type}::operator+{r_type}");

            let method = realm.read().unwrap().lookup(&method_name);

            if let Some(method) = method {
                call_func(realm, method, &[lhs, rhs])
            } else {
                panic!("Incompatible types for operation `+`: `{l_type:?}` and `{r_type:?}`")
            }
        }
        ExprKind::Sub(lhs, rhs) => {
            let lhs = evaluate_expression(Arc::clone(&realm), lhs, true)
                .expect("Cannot be evaluated (lhs)");
            let rhs = evaluate_expression(Arc::clone(&realm), rhs, true)
                .expect("Cannot be evaluated (rhs)");

            let l_type = types::value_to_internal_type(&lhs).unwrap();
            let r_type = types::value_to_internal_type(&rhs).unwrap();

            let method_name = format!("{l_type}::operator-{r_type}");

            let method = realm.read().unwrap().lookup(&method_name);

            if let Some(method) = method {
                call_func(realm, method, &[lhs, rhs])
            } else {
                panic!("Incompatible types for operation `-`: `{l_type:?}` and `{r_type:?}`")
            }
        }
        ExprKind::Mul(lhs, rhs) => {
            let lhs = evaluate_expression(Arc::clone(&realm), lhs, true)
                .expect("Cannot be evaluated (lhs)");
            let rhs = evaluate_expression(Arc::clone(&realm), rhs, true)
                .expect("Cannot be evaluated (rhs)");

            let l_type = types::value_to_internal_type(&lhs).unwrap();
            let r_type = types::value_to_internal_type(&rhs).unwrap();

            let method_name = format!("{l_type}::operator*{r_type}");

            let method = realm.read().unwrap().lookup(&method_name);

            if let Some(method) = method {
                call_func(realm, method, &[lhs, rhs])
            } else {
                panic!("Incompatible types for operation `*`: `{l_type:?}` and `{r_type:?}`")
            }
        }
        ExprKind::Div(lhs, rhs, division_kind) => {
            let lhs = evaluate_expression(Arc::clone(&realm), lhs, true)
                .expect("Cannot be evaluated (lhs)");
            let rhs = evaluate_expression(Arc::clone(&realm), rhs, true)
                .expect("Cannot be evaluated (rhs)");

            let l_type = types::value_to_internal_type(&lhs).unwrap();
            let r_type = types::value_to_internal_type(&rhs).unwrap();

            let op = match division_kind {
                DivisionKind::Neutral => "/",
                DivisionKind::RoundingUp => "/+",
                DivisionKind::RoundingDown => "/-",
            };

            let method_name = format!("{l_type}::operator{op}{r_type}");

            let method = realm.read().unwrap().lookup(&method_name);

            if let Some(method) = method {
                call_func(realm, method, &[lhs, rhs])
            } else {
                panic!("Incompatible types for operation `{op}`: `{l_type:?}` and `{r_type:?}`")
            }
        }
        ExprKind::Mod(lhs, rhs) => {
            let lhs = evaluate_expression(Arc::clone(&realm), lhs, true)
                .expect("Cannot be evaluated (lhs)");
            let rhs = evaluate_expression(Arc::clone(&realm), rhs, true)
                .expect("Cannot be evaluated (rhs)");

            let l_type = types::value_to_internal_type(&lhs).unwrap();
            let r_type = types::value_to_internal_type(&rhs).unwrap();

            let method_name = format!("{l_type}::operator%{r_type}");

            let method = realm.read().unwrap().lookup(&method_name);

            if let Some(method) = method {
                call_func(realm, method, &[lhs, rhs])
            } else {
                panic!("Incompatible types for operation `%`: `{l_type:?}` and `{r_type:?}`")
            }
        },
        ExprKind::And(lhs, rhs) => todo!(),
        ExprKind::Or(lhs, rhs) => todo!(),
        ExprKind::BitAnd(spanned, spanned1) => todo!(),
        ExprKind::BitOr(spanned, spanned1) => todo!(),
        ExprKind::BitShiftLeft(spanned, spanned1) => todo!(),
        ExprKind::BitShiftRight(spanned, spanned1) => todo!(),
        ExprKind::Equals(spanned, spanned1) => todo!(),
        ExprKind::Greater(spanned, spanned1) => todo!(),
        ExprKind::GreaterOrEquals(spanned, spanned1) => todo!(),
        ExprKind::Less(spanned, spanned1) => todo!(),
        ExprKind::LessOrEquals(spanned, spanned1) => todo!(),
        ExprKind::Not(spanned) => todo!(),
        ExprKind::Neg(spanned) => todo!(),
        ExprKind::Identifier(id) => {
            let value = realm.read().unwrap().lookup(id.as_str());

            if value.is_none() {
                panic!("Name `{id}` is not defined.");
            }

            value
        }
        ExprKind::Number(nr) => {
            let is_float = nr.contains('.');

            let val = if is_float {
                Value::Float(nr.parse::<f64>().unwrap())
            } else {
                Value::Integer(nr.parse::<i128>().unwrap())
            };

            Some(val)
        }
        ExprKind::String(st) => Some(Value::String(Arc::new(st.clone()))),
        ExprKind::Block(ast) => {
            // let new_realm = Realm {
            //     values: HashMap::new(),
            //     parent: Some(realm),
            // };

            // Some(exec_inner(Arc::new(RwLock::new(new_realm)), ast))

            Some(exec_inner(Arc::clone(&realm), ast))
        }
        ExprKind::Array(spanneds) => todo!(),
        ExprKind::Call { callee, parameters } => {
            let func_name = callee.value.as_id().expect(&format!(
                "The value (callee name) is not an identifier! ({:?})",
                callee.value
            ));

            let func = realm.write().unwrap().lookup(func_name);

            if func.is_none() {
                panic!("Undefined function: {func_name:#?}");
            }

            let args: Vec<Value> = parameters
                .iter()
                .map(|x| evaluate_expression(Arc::clone(&realm), x, true).unwrap_or(Value::Nil))
                .collect();

            call_func(realm, func.unwrap(), &args)
        }
        ExprKind::Assignment { name, value } => {
            let lhs = name.value.as_id().unwrap();
            let rhs = evaluate_expression(Arc::clone(&realm), value, true).unwrap();

            realm
                .write()
                .unwrap()
                .values_mut()
                .insert(lhs.to_owned(), rhs.clone());

            if is_subexpression { Some(rhs) } else { None }
        }
        ExprKind::PropertyAccess { origin, property } => todo!(),
        ExprKind::IndexedAccess { origin, index } => todo!(),
    }
}

// Performs a function call.
// Supported both native and regular functions.
fn call_func(realm: SharedRealm, func: Value, args: &[Value]) -> Option<Value> {
    let mut new_realm = Realm {
        values: HashMap::new(),
        parent: Some(realm),
    };

    if let Value::Native(native) = func {
        return Some(native(&mut new_realm, args));
    }

    if let Value::Function(func) = func {
        let parameters = &func.params;

        if parameters.len() != args.len() {
            panic!("Insufficent arguments!");
        }

        // Arguments are just temporary variables

        new_realm.values = func.closure_realm.read().unwrap().values().clone();

        for (par, arg) in parameters.iter().zip(args) {
            new_realm.values_mut().insert(par.clone(), arg.clone());
        }

        return exec_single_statement(Arc::new(RwLock::new(new_realm)), &func.body);
    }

    None
}
