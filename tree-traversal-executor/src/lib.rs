use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use flylang_common::{source::Source, spanned::Spanned};
use flylang_diagnostics::additions::Note;
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
pub mod parser_glue;
pub mod realm;
pub mod runtime;
pub mod types;

pub type SharedRealm = Arc<RwLock<Realm>>;

enum ModuleState {
    Loading,
    Loaded(LoadedModule),
}

struct LoadedModule {
    exports: HashMap<String, Value>,
}

pub struct Interpreter {
    // "Root" Realm of the interpreter
    world: SharedRealm,

    // A realm that contains only internal modules and functions.
    builtins: SharedRealm,

    // It tracks modules currently in use.
    module_registry: Arc<RwLock<HashMap<PathBuf, ModuleState>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut builtins = Realm::new();

        // Import native functions from modules.
        // Chain 'em all!
        let natives = core::iter::empty()
            .chain(runtime::arrays::EXPORT.iter())
            .chain(runtime::booleans::EXPORT.iter())
            .chain(runtime::floats::EXPORT.iter())
            .chain(runtime::integers::EXPORT.iter())
            .chain(runtime::nil::EXPORT.iter())
            .chain(runtime::print::EXPORT.iter())
            .chain(runtime::strings::EXPORT.iter());

        // Import native functions into the builtins subworld.
        for (name, func) in natives {
            builtins
                .values_mut()
                .insert(name.to_string(), Value::Native(*func));
        }

        let builtins = Arc::new(RwLock::new(builtins));
        let world = Arc::new(RwLock::new(Realm::dive(Arc::clone(&builtins))));

        Self {
            builtins,
            world,
            module_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn world(&self) -> &SharedRealm {
        &self.world
    }

    /// Entry point of the interpreter, it accepts a list of statements given by the parser.
    /// Since it accepts any kind of statement including expressions, it will return a value.
    pub fn execute(&self, ast: Vec<Statement>) -> ControlFlow {
        self.exec_inner(Arc::clone(&self.world), &ast, true)
    }

    /// Script version of `Interpreter::execute`. Doesn't break when value is returned.
    pub fn execute_script(&self, ast: Vec<Statement>) -> ControlFlow {
        self.exec_inner(Arc::clone(&self.world), &ast, false)
    }

    /// Trampoline for executor: operate with given realm and the parsed code
    fn exec_inner(
        &self,
        realm: SharedRealm,
        ast: &[Statement],
        return_on_value: bool,
    ) -> ControlFlow {
        for i in ast {
            let stmt = self.exec_single_statement(Arc::clone(&realm), i);

            debug!("Got: {i:?} => {stmt:?}");

            match stmt {
                Some(cf @ ControlFlow::Return(_)) => return cf,
                Some(cf @ ControlFlow::Break) => return cf,
                Some(cf @ ControlFlow::Continue) => return cf,

                Some(cf @ ControlFlow::Value(_)) if return_on_value => return cf,

                Some(ControlFlow::Value(_)) => continue,
                Some(ControlFlow::Nothing) => continue,
                None => continue,
            }
        }

        debug!("Nothing returned");

        ControlFlow::Nothing
    }

    fn import_module(&self, realm: SharedRealm, path_segments: Vec<String>) {
        eprintln!("{path_segments:?}");

        if path_segments.len() > 1 {
            todo!("Deeper import is not supported yet...");
        }

        let module_name = path_segments.join("::");
        let filename = path_segments[0].clone() + ".fly";

        let path = PathBuf::from(filename.clone());

        if let Some(val) = self.module_registry.read().unwrap().get(&path) {
            match val {
                ModuleState::Loading => panic!("Circular import detected for module: {}", filename),
                ModuleState::Loaded(_) => return, // We don't have to load it again
            }
        }

        self.module_registry
            .write()
            .unwrap()
            .insert(path.clone(), ModuleState::Loading);

        let code = match std::fs::read_to_string(&filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file `{filename}`: {e:?}");
                std::process::exit(1);
            }
        };

        let ast = parser_glue::parse_into_ast(Source::new(filename, code)).unwrap();

        let module_realm = Arc::new(RwLock::new(Realm::dive(Arc::clone(&self.builtins))));

        self.exec_inner(Arc::clone(&module_realm), &ast, false);

        let exports = module_realm.read().unwrap().values().clone();

        self.module_registry.write().unwrap().insert(
            path.clone(),
            ModuleState::Loaded(LoadedModule {
                exports: exports.clone(),
            }),
        );

        for (name, value) in exports {
            realm
                .write()
                .unwrap()
                .values_mut()
                .insert(format!("{}::{}", module_name, name), value);
        }
    }

    /// Execute the single statement.
    /// Not all statements are expressions so they can't return a value.
    /// This is why I make it return `Option<Value>`.
    ///
    /// `return nil` will return `Some(Value::Nil)`
    fn exec_single_statement(
        &self,
        realm: SharedRealm,
        statement: &Statement,
    ) -> Option<ControlFlow> {
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
                let cond = self.evaluate_expression(Arc::clone(&realm), &stmt.condition, false);

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
                        // let inner_realm = Arc::new(RwLock::new(Realm::dive(Arc::clone(&realm))));

                        // return Some(self.exec_inner(inner_realm, &bk));
                        return Some(self.exec_inner(Arc::clone(&realm), &bk, false));
                    } else {
                        panic!("Expected a block!")
                    }

                    // ...
                } else {
                    if let Some(else_body) = &stmt.else_body {
                        // let inner_realm = Arc::new(RwLock::new(Realm::dive(Arc::clone(&realm))));

                        // self.exec_single_statement(inner_realm, &else_body);
                        self.exec_single_statement(realm, &else_body)
                    } else {
                        Some(ControlFlow::Nothing)
                    }
                }
            }
            Statement::ModuleUsageDeclaration { path } => {
                self.import_module(realm, self.path_segments_to_vec(path));

                Some(ControlFlow::Nothing)
            }
            Statement::Scope { .. } => todo!(),
            Statement::Return { value } => {
                let cf = self.evaluate_expression(realm, value, false);

                debug!("Return: {cf:?}");

                let ControlFlow::Value(v) = cf else {
                    panic!("Expected a value in return statement, got: {cf:?}");
                };

                Some(ControlFlow::Return(v))
            }
            Statement::Expr(expr) => {
                debug!("Evaluating: {expr:?}");

                let expr = self.evaluate_expression(realm, expr, false);

                debug!("Expression: {expr:?}");

                Some(expr)
            }
            Statement::While(while_loop) => {
                let While { condition, body } = while_loop;

                loop {
                    // while x < n { ...
                    //       ^^^^^
                    // Values are accessed outside the while body's scope, so passing `realm` is OK.
                    let cond = self.evaluate_expression(Arc::clone(&realm), condition, false);

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
                        let block_result = self.exec_inner(Arc::clone(&realm), &bk, false);

                        match block_result {
                            ControlFlow::Return(_) => return Some(block_result),
                            ControlFlow::Break => break,
                            ControlFlow::Continue => continue,
                            _ => (),
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
        &self,
        realm: SharedRealm,
        expr: &Expression,
        is_subexpression: bool,
    ) -> ControlFlow {
        let Spanned { value, address } = expr;

        debug!("Eval: {expr:?}");

        match value {
            ExprKind::Add(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "+", lhs, rhs).unwrap())
            }
            ExprKind::Sub(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "-", lhs, rhs).unwrap())
            }
            ExprKind::Mul(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "*", lhs, rhs).unwrap())
            }
            ExprKind::Div(lhs, rhs, division_kind) => {
                let op = match division_kind {
                    DivisionKind::Neutral => "/",
                    DivisionKind::RoundingUp => "/+",
                    DivisionKind::RoundingDown => "/-",
                };

                ControlFlow::Value(self.binary_op_helper(realm, op, lhs, rhs).unwrap())
            }
            ExprKind::Mod(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "%", lhs, rhs).unwrap())
            }
            ExprKind::And(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "&&", lhs, rhs).unwrap())
            }
            ExprKind::Or(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "||", lhs, rhs).unwrap())
            }
            ExprKind::BitAnd(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "&", lhs, rhs).unwrap())
            }
            ExprKind::BitOr(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "|", lhs, rhs).unwrap())
            }
            ExprKind::BitShiftLeft(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "<<", lhs, rhs).unwrap())
            }
            ExprKind::BitShiftRight(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, ">>", lhs, rhs).unwrap())
            }
            ExprKind::Equals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "==", lhs, rhs).unwrap())
            }
            ExprKind::Greater(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, ">", lhs, rhs).unwrap())
            }
            ExprKind::GreaterOrEquals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, ">=", lhs, rhs).unwrap())
            }
            ExprKind::Less(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "<", lhs, rhs).unwrap())
            }
            ExprKind::LessOrEquals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "<=", lhs, rhs).unwrap())
            }
            ExprKind::Not(val) => {
                ControlFlow::Value(self.unary_op_helper(realm, "!", val).unwrap())
            }
            ExprKind::Neg(val) => {
                ControlFlow::Value(self.unary_op_helper(realm, "-", val).unwrap())
            }
            ExprKind::Identifier(id) => {
                debug!("Looking for {id:#?} from realm module ...");

                let value = realm.read().unwrap().lookup(id.as_str());

                if value.is_none() {
                    flylang_diagnostics::Diagnostics {}.error(
                        &format!("Name `{id}` is not defined."),
                        &address,
                        &[
                            Note::new(address.clone(), "here")
                        ],
                        &[],
                    );
                    std::process::exit(1);
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
                let block_result = self.exec_inner(inner_realm, ast, false);

                match block_result {
                    ControlFlow::Return(_) => block_result,
                    ControlFlow::Value(v) => ControlFlow::Value(v),
                    ControlFlow::Nothing => ControlFlow::Nothing,
                    other => other,
                }
            }
            ExprKind::Array(exprs) => {
                let values: Vec<Value> = exprs
                    .iter()
                    .map(|x| {
                        let expr = self.evaluate_expression(Arc::clone(&realm), x, false);

                        let ControlFlow::Value(value) = expr else {
                            panic!("Expected value, got: {expr:?}");
                        };

                        value
                    })
                    .collect();

                ControlFlow::Value(Value::Array(Arc::new(Mutex::new(values))))
            }
            ExprKind::Call { callee, parameters } => {
                // Special case - method call by using property access.
                if let ExprKind::PropertyAccess { origin, property } = &callee.value {
                    let obj = self.evaluate_expression(Arc::clone(&realm), origin, true);
                    let ControlFlow::Value(obj) = obj else {
                        panic!()
                    };

                    let prop = property.value.as_id().unwrap();
                    let type_name = types::value_to_internal_type(&obj).unwrap();
                    let method_key = format!("{type_name}::{prop}");

                    let method = realm
                        .read()
                        .unwrap()
                        .lookup(&method_key)
                        .unwrap_or_else(|| panic!("No method `{prop}` on `{type_name}`"));

                    let mut args = vec![obj]; // receiver (self) is first argument
                    for p in parameters {
                        let ControlFlow::Value(v) =
                            self.evaluate_expression(Arc::clone(&realm), p, true)
                        else {
                            panic!()
                        };
                        args.push(v);
                    }

                    return self.call_func(realm, method, &args);
                }

                let func = self.evaluate_expression(Arc::clone(&realm), callee, true);

                let ControlFlow::Value(func) = func else {
                    panic!("Expected a function as value, got: {func:?}");
                };

                let args: Vec<Value> = parameters
                    .iter()
                    .map(|x| {
                        let expr = self.evaluate_expression(Arc::clone(&realm), x, true);

                        if let ControlFlow::Value(va) = expr {
                            va
                        } else {
                            panic!("Expected value, got: {expr:?}");
                        }
                    })
                    .collect();

                debug!("Calling func with args: {:?}", args);

                self.call_func(realm, func, &args)
            }
            ExprKind::Assignment { name, value } => {
                let target = self.resolve_lvalue(Arc::clone(&realm), name);
                let rhs = self.evaluate_expression(Arc::clone(&realm), value, true);

                let ControlFlow::Value(rhs) = rhs else {
                    panic!("Expected RHS as value, got: {rhs:?}");
                };

                self.assign(Arc::clone(&realm), target, rhs.clone());

                if is_subexpression {
                    ControlFlow::Value(rhs)
                } else {
                    ControlFlow::Nothing
                }
            }
            ExprKind::PropertyAccess { origin, property } => {
                let obj = self.evaluate_expression(Arc::clone(&realm), origin, true);
                let ControlFlow::Value(obj) = obj else {
                    panic!("Expected value")
                };

                let prop = property.value.as_id().unwrap();
                let type_name = types::value_to_internal_type(&obj).unwrap();
                let method_key = format!("{type_name}::{prop}");

                let val = realm.read().unwrap().lookup(&method_key);

                debug!("Property: {prop:?}");

                match val {
                    Some(val) => {
                        return ControlFlow::Value(val);
                    }
                    None => panic!("No property `{prop}` on type `{type_name}`"),
                }
            }
            ExprKind::IndexedAccess { origin, index } => {
                let container = self.evaluate_expression(Arc::clone(&realm), origin, true);
                let index = self.evaluate_expression(Arc::clone(&realm), index, true);

                let ControlFlow::Value(container) = container else {
                    panic!("Expected value")
                };

                let ControlFlow::Value(index) = index else {
                    panic!("Expected value")
                };

                // Now it gets interesting

                match (container, index) {
                    (Value::Array(arr), Value::Integer(i)) => {
                        let arr = arr.lock().unwrap();

                        let val: Option<&Value> = arr.get(i as usize);

                        match val {
                            Some(v) => ControlFlow::Value(v.clone()),
                            None => panic!("Index {i} out of bounds (len {})", arr.len()),
                        }
                    }
                    (Value::String(s), Value::Integer(i)) => {
                        // character access
                        match s.chars().nth(i as usize) {
                            Some(c) => ControlFlow::Value(Value::String(Arc::new(c.to_string()))),
                            None => panic!("String index {i} out of bounds"),
                        }
                    }
                    (container, index) => panic!(
                        "Cannot index into {:?} with {:?}",
                        types::value_to_internal_type(&container),
                        types::value_to_internal_type(&index)
                    ),
                }
            }
            ExprKind::AddAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "+", lhs, rhs, is_subexpression)
            }
            ExprKind::SubAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "-", lhs, rhs, is_subexpression)
            }
            ExprKind::MulAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "*", lhs, rhs, is_subexpression)
            }
            ExprKind::DivAssign(lhs, rhs, division_kind) => {
                let op = match division_kind {
                    DivisionKind::Neutral => "/",
                    DivisionKind::RoundingUp => "/+",
                    DivisionKind::RoundingDown => "/-",
                };

                self.compound_assignment_helper(realm, op, lhs, rhs, is_subexpression)
            }
            ExprKind::ModAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "%", lhs, rhs, is_subexpression)
            }

            ExprKind::NotEquals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "!=", lhs, rhs).unwrap())
            }

            ExprKind::Path { .. } => {
                let key = self.flatten_path(expr);

                let result = realm.read().unwrap().lookup(&key);

                match result {
                    Some(val) => ControlFlow::Value(val),
                    None => panic!("Undefined path: `{key}`"),
                }
            }

            ExprKind::True => ControlFlow::Value(Value::Bool(true)),
            ExprKind::False => ControlFlow::Value(Value::Bool(false)),
        }
    }

    // Performs a function call.
    // Supported both native and regular functions.
    fn call_func(&self, realm: SharedRealm, func: Value, args: &[Value]) -> ControlFlow {
        debug!("Call function with parameters {args:?}");

        if let Value::Native(native) = func {
            let new_realm = Realm::dive(realm);

            return native(self, Arc::new(RwLock::new(new_realm)), args);
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

            let result = self
                .exec_single_statement(Arc::new(RwLock::new(new_realm)), &func.body)
                .unwrap_or(ControlFlow::Nothing);

            debug!(
                "Executing func with params {:?} returned {:?}",
                func.params, result
            );

            return match result {
                ControlFlow::Return(v) => ControlFlow::Value(v),
                other => other,
            };
        }

        ControlFlow::Nothing
    }

    pub fn call_func_extern(&self, name: &str, args: &[Value]) -> Option<ControlFlow> {
        let method = self.world.read().unwrap().lookup(name)?;

        if let Value::Native(native) = method {
            let new_realm = Realm::dive(Arc::clone(&self.world));

            return Some(native(self, Arc::new(RwLock::new(new_realm)), args));
        }

        if let Value::Function(func) = method {
            let mut new_realm = Realm::dive(Arc::clone(&func.closure_realm));

            let parameters = &func.params;

            if parameters.len() != args.len() {
                panic!("Insufficent arguments!");
            }

            // Arguments are just temporary variables

            for (par, arg) in parameters.iter().zip(args) {
                new_realm.values_mut().insert(par.clone(), arg.clone());
            }

            let result = self
                .exec_single_statement(Arc::new(RwLock::new(new_realm)), &func.body)
                .unwrap_or(ControlFlow::Nothing);

            return Some(match result {
                ControlFlow::Return(v) => ControlFlow::Value(v),
                other => other,
            });
        }

        None
    }

    fn binary_op_helper(
        &self,
        realm: SharedRealm,
        op: &str,
        lhs: &Expression,
        rhs: &Expression,
    ) -> Option<Value> {
        let lhs_val = self.evaluate_expression(Arc::clone(&realm), lhs, true);
        let rhs_val = self.evaluate_expression(Arc::clone(&realm), rhs, true);

        let ControlFlow::Value(lhs_val) = lhs_val else {
            panic!("A value should be returned by LHS, got: {lhs_val:?}");
        };

        let ControlFlow::Value(rhs_val) = rhs_val else {
            panic!("A value should be returned by RHS, got: {rhs_val:?}");
        };

        self.binary_op_helper_values(realm, op, lhs_val, rhs_val)
    }

    fn binary_op_helper_values(
        &self,
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
            if let ControlFlow::Value(va) = self.call_func(realm, method, &[lhs, rhs]) {
                return Some(va);
            } else {
                panic!("Failed to get a return value from function call.");
            }
        } else {
            panic!("Incompatible types for operation `{op}`: `{l_type}` and `{r_type}`")
        }
    }

    fn unary_op_helper(&self, realm: SharedRealm, op: &str, expr: &Expression) -> Option<Value> {
        let expr_val = self.evaluate_expression(Arc::clone(&realm), expr, true);

        let ControlFlow::Value(expr_val) = expr_val else {
            panic!("A value should be returned, got: {expr_val:?}");
        };

        let ty = types::value_to_internal_type(&expr_val).unwrap();

        let method_name = format!("{ty}::operator{op}");

        let method = realm
            .read()
            .unwrap()
            .lookup(&method_name)
            .unwrap_or_else(|| panic!("Incompatible type for operation `{op}`: `{ty}`"));

        if let ControlFlow::Value(va) = self.call_func(realm, method, &[expr_val]) {
            return Some(va);
        } else {
            panic!("Failed to get a return value from function call.");
        }
    }

    fn resolve_lvalue(&self, realm: SharedRealm, expr: &Expression) -> LValue {
        match &expr.value {
            ExprKind::Identifier(name) => LValue::Identifier(name.clone()),

            ExprKind::IndexedAccess { origin, index } => {
                let container = self.evaluate_expression(Arc::clone(&realm), origin, true);
                let index = self.evaluate_expression(Arc::clone(&realm), index, true);

                let ControlFlow::Value(container) = container else {
                    panic!("Expected value as container, got: {container:?}");
                };

                let ControlFlow::Value(index) = index else {
                    panic!("Expected value as index, got: {index:?}");
                };

                LValue::Index { container, index }
            }

            ExprKind::PropertyAccess { origin, property } => {
                let object = self.evaluate_expression(Arc::clone(&realm), origin, true);
                let name = property.value.as_id().unwrap().to_owned();

                let ControlFlow::Value(object) = object else {
                    panic!("Expected value as object, got: {object:?}");
                };

                LValue::Property { object, name }
            }

            _ => panic!("Invalid assignment target"),
        }
    }

    fn read_lvalue(&self, realm: SharedRealm, target: &LValue) -> Value {
        match target {
            LValue::Identifier(name) => realm.read().unwrap().lookup(name.as_str()).unwrap(),
            LValue::Index { container, index } => {
                let Value::Array(arr) = container else {
                    panic!("Cannot index into type: {:?}", container);
                };

                let Value::Integer(i) = index else {
                    panic!("Type `{:?}` cannot be used as an index", index);
                };

                arr.lock().unwrap()[*i as usize].clone()
            }
            LValue::Property { .. } => todo!("Property value read"),
        }
    }

    // I separated it into a function to make it compatible with OpAssignments (+=, -=, ...)
    fn assign(&self, realm: SharedRealm, target: LValue, value: Value) {
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
            LValue::Property { .. } => {
                todo!("Property assignment when I add objects")
            }
        }
    }

    fn compound_assignment_helper(
        &self,
        realm: SharedRealm,
        op: &str,
        name: &Expression,
        value: &Expression,
        is_subexpression: bool,
    ) -> ControlFlow {
        let target = self.resolve_lvalue(Arc::clone(&realm), name);

        // Read current value — identifier needs a lookup, indexed needs evaluation
        let current = self.read_lvalue(Arc::clone(&realm), &target);
        let rhs = self.evaluate_expression(Arc::clone(&realm), value, true);

        let ControlFlow::Value(rhs) = rhs else {
            panic!("Expected RHS as value, got: {rhs:?}");
        };

        let result = self
            .binary_op_helper_values(Arc::clone(&realm), op, current, rhs)
            .unwrap();

        self.assign(Arc::clone(&realm), target, result.clone());

        if is_subexpression {
            ControlFlow::Value(result)
        } else {
            ControlFlow::Nothing
        }
    }

    fn path_segments_to_vec(&self, expr: &Expression) -> Vec<String> {
        match &expr.value {
            ExprKind::Path { parent, value } => {
                let lhs = self.path_segments_to_vec(parent);
                let rhs = self.path_segments_to_vec(value);

                lhs.into_iter()
                    .chain(rhs.into_iter())
                    .collect::<Vec<String>>()
            }
            ExprKind::Identifier(name) => vec![name.clone()],
            _ => panic!("Invalid path segment: {:?}", expr.value),
        }
    }

    fn property_access_segments_to_vec(&self, expr: &Expression) -> Vec<String> {
        match &expr.value {
            ExprKind::PropertyAccess { origin, property } => {
                let lhs = self.property_access_segments_to_vec(origin);
                let rhs = self.property_access_segments_to_vec(property);

                lhs.into_iter()
                    .chain(rhs.into_iter())
                    .collect::<Vec<String>>()
            }
            ExprKind::Identifier(name) => vec![name.clone()],
            _ => panic!("Invalid property access segment: {:?}", expr.value),
        }
    }

    // Flattens path into a string that will be used in Realm's HashMap lookup.
    fn flatten_path(&self, expr: &Expression) -> String {
        self.path_segments_to_vec(expr).join("::")
    }
}
