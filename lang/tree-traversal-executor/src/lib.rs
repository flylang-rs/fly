use std::{
    collections::{HashMap, LinkedList},
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use flylang_common::{Address, source::Source, spanned::Spanned};
use flylang_parser::ast::{DivisionKind, ExprKind, Expression, Statement, While};
use log::debug;

use crate::{
    calltrace::{CallFrame, CallSegment},
    control_flow::ControlFlow,
    error::{CallError, InterpreterError},
    object::{
        Value,
        function::{Function, FunctionNameKind},
        lvalue::LValue,
        module::Module,
        record::{Record, RecordField, RecordInstance, RecordInstanceField},
    },
    realm::Realm,
};

#[cfg(test)]
pub mod tests;

pub mod calltrace;
pub mod control_flow;
pub mod error;
pub mod object;
pub mod realm;
pub mod runtime;
pub mod types;

pub type SharedRealm = Arc<RwLock<Realm>>;
pub type InterpreterResult<T> = Result<T, InterpreterError>;

enum ModuleState {
    Loading,
    Loaded, /* (LoadedModule) */
}

// struct LoadedModule {
//     exports: HashMap<String, Value>,
// }

pub struct Interpreter {
    // "Root" Realm of the interpreter
    world: SharedRealm,

    // A realm that contains only internal modules and functions. (actually a root realm)
    builtins: SharedRealm,

    // It tracks modules currently in use.
    module_registry: Arc<RwLock<HashMap<PathBuf, ModuleState>>>,

    // Contains call trace to output it when an error happens.
    call_trace: LinkedList<CallFrame>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut builtins = Realm::new();

        // Import native functions from modules.
        // Chain 'em all!
        let natives = core::iter::empty()
            .chain(runtime::arrays::EXPORT.iter())
            .chain(runtime::exit::EXPORT.iter())
            .chain(runtime::functions::EXPORT.iter())
            .chain(runtime::reals::EXPORT.iter())
            .chain(runtime::nil::EXPORT.iter())
            .chain(runtime::print::EXPORT.iter())
            .chain(runtime::strings::EXPORT.iter())
            .chain(runtime::types::EXPORT.iter());

        // Import native functions into the builtins subworld.
        for (name, func) in natives {
            builtins
                .values_mut()
                .insert(name.to_string(), Value::Native(*func));
        }

        let builtins = Arc::new(RwLock::new(builtins));

        let modules = [
            runtime::booleans::init(&builtins),
            runtime::integers::init(&builtins),
            runtime::strings::init(&builtins)
        ];

        for i in modules {
            debug!(
                "Adding: {} ({:?})",
                &i.name,
                i.realm.read().unwrap().values().keys().collect::<Vec<_>>()
            );

            builtins
                .write()
                .unwrap()
                .values_mut()
                .insert(i.name.clone(), Value::Module(i.into()));
        }

        let world = Arc::new(RwLock::new(Realm::dive(Arc::clone(&builtins))));

        Self {
            builtins,
            world,
            module_registry: Arc::new(RwLock::new(HashMap::new())),
            call_trace: LinkedList::new(),
        }
    }

    pub fn world(&self) -> &SharedRealm {
        &self.world
    }

    pub fn calltrace(&self) -> &LinkedList<CallFrame> {
        &self.call_trace
    }

    /// Entry point of the interpreter, it accepts a list of statements given by the parser.
    /// Since it accepts any kind of statement including expressions, it will return a value.
    pub fn execute(&mut self, ast: Vec<Statement>) -> InterpreterResult<ControlFlow> {
        self.exec_inner(Arc::clone(&self.world), &ast, true)
    }

    /// Script version of `Interpreter::execute`. Doesn't break when value is returned.
    pub fn execute_script(&mut self, ast: Vec<Statement>) -> InterpreterResult<ControlFlow> {
        self.exec_inner(Arc::clone(&self.world), &ast, false)
    }

    /// Trampoline for executor: operate with given realm and the parsed code
    fn exec_inner(
        &mut self,
        realm: SharedRealm,
        ast: &[Statement],
        return_on_value: bool,
    ) -> InterpreterResult<ControlFlow> {
        for i in ast {
            let stmt = self.exec_single_statement(Arc::clone(&realm), i)?;

            debug!("Got: {i:?} => {stmt:?}");

            match stmt {
                cf @ ControlFlow::Return(_) => return Ok(cf),
                cf @ ControlFlow::Break => return Ok(cf),
                cf @ ControlFlow::Continue => return Ok(cf),

                cf @ ControlFlow::Value(_) if return_on_value => return Ok(cf),

                ControlFlow::Value(_) => continue,
                ControlFlow::Nothing => continue,
            }
        }

        debug!("Nothing returned");

        Ok(ControlFlow::Nothing)
    }

    fn import_module(
        &mut self,
        realm: SharedRealm,
        importer: &str,
        path_segments: Vec<String>,
    ) -> InterpreterResult<()> {
        if path_segments.len() > 1 {
            todo!("Deeper import is not supported yet...");
        }

        let module_name = path_segments.join("::");
        let filename = path_segments[0].clone() + ".fly";

        debug!("Importer: {importer:?}");

        let mut path = PathBuf::from(importer)
            .parent()
            .map(|x| x.to_path_buf())
            .unwrap(); // TODO: Checks

        path = path.join(&filename);

        debug!("Final path: {path:?}");

        if let Some(val) = self.module_registry.read().unwrap().get(&path) {
            match val {
                ModuleState::Loading => panic!("Circular import detected for module: {}", filename),
                ModuleState::Loaded /*(_)*/ => return Ok(()), // We don't have to load it again
            }
        }

        self.module_registry
            .write()
            .unwrap()
            .insert(path.clone(), ModuleState::Loading);

        let code = match std::fs::read_to_string(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file `{filename}`: {e:?}");
                std::process::exit(1);
            }
        };

        let ast =
            flylang_lexparse_glue::parse_source(Arc::new(Source::new(filename, code))).unwrap();

        let module_realm = Arc::new(RwLock::new(Realm::dive(Arc::clone(&self.builtins))));

        self.exec_inner(Arc::clone(&module_realm), &ast, false)?;

        // let exports = module_realm.read().unwrap().values().clone();

        self.module_registry.write().unwrap().insert(
            path.clone(),
            ModuleState::Loaded, /* (LoadedModule {
                                     exports: exports.clone(),
                                 }) */
        );

        realm.write().unwrap().values_mut().insert(
            module_name.clone(),
            Value::Module(Arc::new(Module {
                name: module_name,
                realm: module_realm,
            })),
        );

        Ok(())
    }

    /// Execute the single statement.
    fn exec_single_statement(
        &mut self,
        realm: SharedRealm,
        statement: &Statement,
    ) -> InterpreterResult<ControlFlow> {
        match statement {
            Statement::Function(function) => {
                let real_name: Spanned<String> = match &function.name.value {
                    ExprKind::Identifier(id) => {
                        Spanned::new(id.clone(), function.name.address.clone())
                    }
                    ExprKind::Path { .. } => Spanned::new(
                        self.path_segments_to_vec(&function.name).join("::"),
                        function.name.address.clone(),
                    ),
                    fna => todo!("Function name is complex: {fna:?}"),
                };

                // Note: it can't be mixed up with module path notation, because we're working with
                // function definition.
                let record_path = match &function.name.value {
                    ExprKind::Path { .. } => Some(self.path_segments_to_vec(&function.name)),
                    _ => None,
                };

                let mut params: Vec<String> = function
                    .arguments
                    .iter()
                    .map(|x| x.value.as_id().map(str::to_owned).unwrap())
                    .collect();

                // If a function belongs to method, add `self` as an object receiver
                if record_path.is_some() && !function.is_static {
                    params.insert(0, "self".to_string());
                }

                let value = Value::Function(Arc::new(Function {
                    normal_name: FunctionNameKind::Normal(real_name.clone()),
                    params,
                    body: *function.body.clone(),
                    closure_realm: Arc::clone(&realm),
                }));

                debug!("Record name: {:?}", record_path);

                // If it's a record method, add function to its fields instead.
                if let Some(stems) = &record_path {
                    let record = realm
                        .read()
                        .unwrap()
                        .values()
                        .get(&stems[0])
                        .and_then(|x| x.as_record())
                        .unwrap_or_else(|| panic!("Failed to resolve record!"));

                    record
                        .methods
                        .write()
                        .unwrap()
                        .insert(stems.iter().last().unwrap().to_string(), value);
                } else {
                    realm
                        .write()
                        .unwrap()
                        .values_mut()
                        .insert(real_name.value, value);
                }

                Ok(ControlFlow::Nothing)
            }
            Statement::If(stmt) => {
                // if x < n { ...
                //       ^^^^^
                // Values are accessed outside the `if` body's scope, so passing `realm` is OK.
                let cond = self.evaluate_expression(Arc::clone(&realm), &stmt.condition, false)?;

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
                        return self.exec_inner(Arc::clone(&realm), &bk, false);
                    } else {
                        panic!("Expected a block!")
                    }

                    // ...
                } else {
                    if let Some(else_body) = &stmt.else_body {
                        self.exec_single_statement(realm, else_body)
                    } else {
                        Ok(ControlFlow::Nothing)
                    }
                }
            }
            Statement::ModuleUsageDeclaration { path } => {
                // Importer = who imports the module.
                // Sly and maybe forbidden way to do it is get importer filepath from a token that decalres module import.
                let importer = path.address.source.filepath.as_str();

                self.import_module(realm, importer, self.path_segments_to_vec(path))?;

                Ok(ControlFlow::Nothing)
            }
            Statement::Scope { .. } => todo!(),
            Statement::Return { value } => {
                let cf = self.evaluate_expression(realm, value, false)?;

                debug!("Return: {cf:?}");

                let ControlFlow::Value(v) = cf else {
                    panic!("Expected a value in return statement, got: {cf:?}");
                };

                Ok(ControlFlow::Return(v))
            }
            Statement::Expr(expr) => {
                debug!("Evaluating: {expr:?}");

                let expr = self.evaluate_expression(realm, expr, false)?;

                debug!("Expression: {expr:?}");

                Ok(expr)
            }
            Statement::While(while_loop) => {
                let While { condition, body } = while_loop;

                loop {
                    // while x < n { ...
                    //       ^^^^^
                    // Values are accessed outside the while body's scope, so passing `realm` is OK.
                    let cond = self.evaluate_expression(Arc::clone(&realm), condition, false)?;

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
                        let block_result = self.exec_inner(Arc::clone(&realm), bk, false)?;

                        match block_result {
                            ControlFlow::Return(_) => return Ok(block_result),
                            ControlFlow::Break => break,
                            ControlFlow::Continue => continue,
                            _ => (),
                        }
                    } else {
                        panic!("Expected a block!")
                    }
                }

                Ok(ControlFlow::Nothing)
            }
            Statement::Continue => Ok(ControlFlow::Continue),
            Statement::Break => Ok(ControlFlow::Break),
            Statement::VariableDefinition(var) => {
                let lhs = self.resolve_lvalue(
                    Arc::clone(&realm),
                    &var.name.clone().map(ExprKind::Identifier),
                )?;

                let target = match lhs {
                    LValue::Identifier(id) => LValue::PrivateIdentifier(id),
                    piv @ LValue::PrivateIdentifier(_) => piv,
                    _ => unreachable!("Cannot do private indexed or property assignments."),
                };

                let rhs = self.evaluate_expression(
                    Arc::clone(&realm),
                    var.value.as_ref().unwrap(),
                    false,
                )?;

                let ControlFlow::Value(rhs) = rhs else {
                    panic!("Expected RHS as value, got: {rhs:?}");
                };

                self.assign(Arc::clone(&realm), target, rhs.clone());

                Ok(ControlFlow::Nothing)
            }
            Statement::RecordDefinition(record) => {
                let name = &record.name.value;
                let fields = record
                    .fields
                    .value
                    .iter()
                    .map(|x| match x {
                        Statement::VariableDefinition(var) => {
                            let vis = var.visibility;
                            let name = var.name.value.clone();

                            RecordField {
                                name,
                                visibility: vis,
                            }
                        }
                        a => {
                            unreachable!("Record field kind check is done in parser. Found: {a:?}")
                        }
                    })
                    .collect::<Vec<RecordField>>();

                let value = Record {
                    name: name.clone(),
                    fields,
                    methods: Arc::new(RwLock::new(HashMap::new())),
                    definition_realm: Arc::clone(&realm),
                };

                realm
                    .write()
                    .unwrap()
                    .values_mut()
                    .insert(name.clone(), Value::Record(value.into()));

                Ok(ControlFlow::Nothing)
            } // st => todo!("Unexpected statement: {:?}", st),
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
        &mut self,
        realm: SharedRealm,
        expr: &Expression,
        is_subexpression: bool,
    ) -> InterpreterResult<ControlFlow> {
        let Spanned {
            value: expression_kind,
            address,
        } = expr;

        debug!("Eval: {expr:?}");

        let result = match expression_kind {
            ExprKind::Add(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "+", lhs, rhs)?.unwrap())
            }
            ExprKind::Sub(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "-", lhs, rhs)?.unwrap())
            }
            ExprKind::Mul(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "*", lhs, rhs)?.unwrap())
            }
            ExprKind::Div(lhs, rhs, division_kind) => {
                let op = match division_kind {
                    DivisionKind::Neutral => "/",
                    DivisionKind::RoundingUp => "/+",
                    DivisionKind::RoundingDown => "/-",
                };

                ControlFlow::Value(self.binary_op_helper(realm, op, lhs, rhs)?.unwrap())
            }
            ExprKind::Mod(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "%", lhs, rhs)?.unwrap())
            }
            ExprKind::And(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "&&", lhs, rhs)?.unwrap())
            }
            ExprKind::Or(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "||", lhs, rhs)?.unwrap())
            }
            ExprKind::BitAnd(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "&", lhs, rhs)?.unwrap())
            }
            ExprKind::BitOr(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "|", lhs, rhs)?.unwrap())
            }
            ExprKind::BitShiftLeft(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "<<", lhs, rhs)?.unwrap())
            }
            ExprKind::BitShiftRight(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, ">>", lhs, rhs)?.unwrap())
            }
            ExprKind::Equals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "==", lhs, rhs)?.unwrap())
            }
            ExprKind::Greater(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, ">", lhs, rhs)?.unwrap())
            }
            ExprKind::GreaterOrEquals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, ">=", lhs, rhs)?.unwrap())
            }
            ExprKind::Less(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "<", lhs, rhs)?.unwrap())
            }
            ExprKind::LessOrEquals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "<=", lhs, rhs)?.unwrap())
            }
            ExprKind::Not(val) => {
                ControlFlow::Value(self.unary_op_helper(realm, "!", val)?.unwrap())
            }
            ExprKind::Neg(val) => {
                ControlFlow::Value(self.unary_op_helper(realm, "-", val)?.unwrap())
            }
            ExprKind::Nil => ControlFlow::Value(Value::Nil),
            ExprKind::Identifier(id) => {
                debug!(
                    "Looking for {id:#?} from realm module {:?}",
                    realm.read().unwrap().values().keys().collect::<Vec<_>>()
                );

                let value = realm.read().unwrap().lookup(id.as_str());

                if value.is_none() {
                    return Err(InterpreterError::NameNotDefined {
                        name: id.clone(),
                        address: address.clone(),
                    });
                }

                ControlFlow::Value(value.unwrap())
            }
            ExprKind::Number(nr) => {
                let is_float = nr.contains('.');

                let val = if is_float {
                    Value::Real(nr.parse::<f64>().unwrap())
                } else {
                    Value::Integer(nr.parse::<i128>().unwrap())
                };

                ControlFlow::Value(val)
            }
            ExprKind::String(st) => ControlFlow::Value(Value::String(Arc::new(st.clone()))),
            ExprKind::Block(ast) => {
                let inner_realm = Arc::new(RwLock::new(Realm::dive(Arc::clone(&realm))));
                let block_result = self.exec_inner(inner_realm, ast, false)?;

                match block_result {
                    ControlFlow::Return(_) => block_result,
                    ControlFlow::Value(v) => ControlFlow::Value(v),
                    ControlFlow::Nothing => ControlFlow::Nothing,
                    other => other,
                }
            }
            ExprKind::Array(exprs) => {
                let values_iter = exprs.iter().map(|x| {
                    let expr = self.evaluate_expression(Arc::clone(&realm), x, false)?;

                    let ControlFlow::Value(value) = expr else {
                        panic!("Expected value, got: {expr:?}");
                    };

                    Ok(value)
                });

                let mut values: Vec<Value> = vec![];

                for i in values_iter {
                    values.push(i?);
                }

                ControlFlow::Value(Value::Array(Arc::new(Mutex::new(values))))
            }
            ExprKind::Call { callee, parameters } => {
                // Special case - method call by using property access.
                if let ExprKind::PropertyAccess { origin, property } = &callee.value {
                    let obj = self.evaluate_expression(Arc::clone(&realm), origin, true)?;
                    let ControlFlow::Value(obj) = obj else {
                        panic!()
                    };

                    let prop = property.value.as_id().unwrap();

                    let type_name = types::value_to_internal_type(&obj).unwrap().to_string();

                    let method: Value = {
                        // If callee is a part of a record method call, get its method.
                        if let Value::RecordInstance(ri) = &obj {
                            ri.read()
                                .unwrap()
                                .record
                                .methods
                                .read()
                                .unwrap()
                                .get(prop)
                                .cloned()
                                .ok_or_else(|| InterpreterError::NoPropertyForType {
                                    typename: type_name.to_string(),
                                    property: prop.to_string(),
                                    callee_address: property.address.clone(),
                                })?
                        } else {
                            // If not, use oldstyle mode, format strings into a path.
                            let method_key = format!("{type_name}::{prop}");

                            let oldstyle_value = realm.read().unwrap().lookup(&method_key);

                            if oldstyle_value.is_none() {
                                realm
                                    .read()
                                    .unwrap()
                                    .lookup(&type_name)
                                    .and_then(|x| x.as_module())
                                    .and_then(|x| x.realm.read().unwrap().lookup(&prop))
                                    .ok_or_else(|| InterpreterError::NoPropertyForType {
                                        typename: type_name.to_string(),
                                        property: prop.to_string(),
                                        callee_address: property.address.clone(),
                                    })?
                            } else {
                                oldstyle_value.unwrap()
                            }
                        }
                    };

                    let mut args = vec![obj]; // receiver (self) is first argument

                    for p in parameters {
                        let ControlFlow::Value(v) =
                            self.evaluate_expression(Arc::clone(&realm), p, true)?
                        else {
                            panic!()
                        };
                        args.push(v);
                    }

                    let method_key = format!("{type_name}::{prop}");

                    self.push_call_frame_for_methodcall(method_key.clone(), callee);

                    let value = self.call_func(realm, Some(&callee.address), &method, &args);

                    self.call_trace.pop_back();

                    return value;
                }

                let func = self.evaluate_expression(Arc::clone(&realm), callee, true)?;

                let ControlFlow::Value(func) = func else {
                    panic!("Expected a function as value, got: {func:?}");
                };

                let args_iter = parameters.iter().map(|x| {
                    let expr = self.evaluate_expression(Arc::clone(&realm), x, true)?;

                    if let ControlFlow::Value(va) = expr {
                        Ok(va)
                    } else {
                        panic!("Expected value, got: {expr:?}");
                    }
                });

                let mut args = vec![];

                for i in args_iter {
                    args.push(i?);
                }

                debug!("Calling func with args: {:?}", args);

                self.push_call_frame(callee, &func);

                let value = self.call_func(realm, Some(&callee.address), &func, &args)?;

                self.call_trace.pop_back();

                value
            }
            ExprKind::Assignment { name, value } => {
                let target = self.resolve_lvalue(Arc::clone(&realm), name)?;
                let rhs = self.evaluate_expression(Arc::clone(&realm), value, true)?;

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
            ExprKind::PropertyAccess { .. } => {
                // let obj = self.evaluate_expression(Arc::clone(&realm), origin, true)?;
                // let ControlFlow::Value(obj) = obj else {
                //     panic!("Expected value")
                // };

                let lhs = self.resolve_lvalue(Arc::clone(&realm), expr)?;

                let value = self.read_lvalue(realm, &lhs);

                return Ok(ControlFlow::Value(value));
            }
            ExprKind::IndexedAccess { origin, index } => {
                let container = self.evaluate_expression(Arc::clone(&realm), origin, true)?;
                let index = self.evaluate_expression(Arc::clone(&realm), index, true)?;

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
                self.compound_assignment_helper(realm, "+", lhs, rhs, is_subexpression)?
            }
            ExprKind::SubAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "-", lhs, rhs, is_subexpression)?
            }
            ExprKind::MulAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "*", lhs, rhs, is_subexpression)?
            }
            ExprKind::DivAssign(lhs, rhs, division_kind) => {
                let op = match division_kind {
                    DivisionKind::Neutral => "/",
                    DivisionKind::RoundingUp => "/+",
                    DivisionKind::RoundingDown => "/-",
                };

                self.compound_assignment_helper(realm, op, lhs, rhs, is_subexpression)?
            }
            ExprKind::ModAssign(lhs, rhs) => {
                self.compound_assignment_helper(realm, "%", lhs, rhs, is_subexpression)?
            }

            ExprKind::NotEquals(lhs, rhs) => {
                ControlFlow::Value(self.binary_op_helper(realm, "!=", lhs, rhs)?.unwrap())
            }

            ExprKind::Path { .. } => {
                debug!(
                    "{:?}",
                    realm.read().unwrap().values().keys().collect::<Vec<_>>()
                );

                let stems = self.path_segments_to_vec(expr);

                let mut node: Option<Value> = None;

                for (idx, stem) in stems.iter().enumerate() {
                    let result_c = || {
                        if let Some(Value::Record(re)) = &node {
                            if let Some((_, value)) = re
                                .methods
                                .read()
                                .unwrap()
                                .iter()
                                .find(|(name, _)| *name == stem)
                            {
                                return Some(value.clone());
                            }
                        }

                        let rd_realm = if let Some(Value::Module(mo)) = &node {
                            &mo.realm
                        } else {
                            &realm
                        };

                        rd_realm.read().unwrap().lookup(&stem)
                    };

                    let result = result_c();

                    debug!("Got result of {stem}: {:?}", result);

                    if let Some(val) = result {
                        node = Some(val);
                    } else {
                        return Err(InterpreterError::NameNotDefined {
                            name: stems[0..=idx].join("::"),
                            address: expr.address.clone(),
                        });
                    }
                }

                assert!(node.is_some(), "Bug: Path stems are somehow empty");

                ControlFlow::Value(node.unwrap())
            }

            ExprKind::True => ControlFlow::Value(Value::Bool(true)),
            ExprKind::False => ControlFlow::Value(Value::Bool(false)),
            ExprKind::AnonymousFunction { arguments, body } => {
                let value = Value::Function(Arc::new(Function {
                    normal_name: FunctionNameKind::Anonymous,
                    params: arguments
                        .iter()
                        .map(|x| x.value.as_id().unwrap().to_owned())
                        .collect(),
                    body: Statement::Expr(*body.clone()),
                    closure_realm: Arc::clone(&realm),
                }));

                ControlFlow::Value(value)
                // todo!("Anonymous functions! ({value:?})")
            }
            ExprKind::New(new_decl) => {
                let name = match &new_decl.name.value {
                    ExprKind::Identifier(id) => id.clone(),
                    ExprKind::Path { .. } => self.flatten_path(&new_decl.name),
                    obj => panic!("Creating new object from `{obj:?}` is not supported (yet)!"),
                };

                let lvalue = self.resolve_lvalue(Arc::clone(&realm), &new_decl.name)?;

                let record_def = self.read_lvalue(Arc::clone(&realm), &lvalue);

                let record_def = match record_def {
                    Value::Record(record) => record,
                    val => todo!("Expected record value, got: {val:?}"),
                };

                let fields_record_provides: Vec<String> =
                    record_def.fields.iter().map(|x| x.name.clone()).collect();
                let fields_we_have: Vec<Spanned<String>> =
                    new_decl.fields.iter().map(|x| x.0.clone()).collect();

                if fields_record_provides.len() != fields_we_have.len() {
                    panic!(
                        "Not enough fields! ({} expected, {} given)",
                        fields_record_provides.len(),
                        fields_we_have.len()
                    );
                }

                for i in &fields_we_have {
                    if !fields_record_provides.contains(&i.value) {
                        panic!(
                            "Record `{}` doesn't have a field named `{}`",
                            &name, i.value
                        );
                    }
                }

                let mut fields: Vec<RecordInstanceField> = Vec::new();

                for (name, value) in new_decl.fields.iter() {
                    let real_name = name.value.to_owned();
                    let real_value = self.exec_single_statement(
                        Arc::clone(&realm),
                        &Statement::Expr(value.clone()),
                    )?;

                    let real_value = match real_value {
                        ControlFlow::Value(v) => v,
                        er => panic!("Invalid expression result: {er:?}"),
                    };

                    fields.push(RecordInstanceField {
                        name: real_name,
                        value: real_value,
                    });
                }

                ControlFlow::Value(Value::RecordInstance(Arc::new(RwLock::new(
                    RecordInstance {
                        record: record_def,
                        fields,
                    },
                ))))
            }
        };

        Ok(result)
    }

    fn push_call_frame(&mut self, callee: &Expression, func: &Value) {
        let name: Spanned<String> = match &func {
            Value::Function(function) => match &function.normal_name {
                FunctionNameKind::Normal(spanned) => spanned.clone(),
                FunctionNameKind::Anonymous => {
                    if let Spanned {
                        value: ExprKind::Identifier(id),
                        address: saddr,
                    } = callee
                    {
                        Spanned::new(id.clone() + " (anonymous)", saddr.clone())
                    } else {
                        Spanned::new(String::from("<anonymous>"), callee.address.clone())
                    }
                }
            },
            Value::Native(_) => match callee {
                Spanned {
                    value: ExprKind::Identifier(id),
                    address: saddr,
                } => Spanned::new(id.clone(), saddr.clone()),
                Spanned {
                    value: ExprKind::Path { .. },
                    address: saddr,
                } => Spanned::new(self.flatten_path(callee), saddr.clone()),
                _ => {
                    todo!("Make a stringified value of native func {callee:?}")
                }
            },
            _ => todo!(),
        };

        let last = self
            .call_trace
            .iter()
            .last()
            .map(|x| x.function_name.clone())
            .unwrap_or_else(|| "<main>".to_string());

        self.call_trace.push_back(CallFrame {
            function_name: name.value, // the function being called
            from: last.to_string().into(),
            call_site: CallSegment {
                address_filename: callee.address.source.filepath.clone(),
                address_line_col: callee
                    .address
                    .source
                    .location(callee.address.span.start)
                    .into(),
            },
        });
    }

    fn push_call_frame_for_methodcall(&mut self, method_key: String, callee: &Expression) {
        let last = self
            .call_trace
            .iter()
            .last()
            .map(|x| x.function_name.clone())
            .unwrap_or_else(|| "<main>".to_string());

        self.call_trace.push_back(CallFrame {
            function_name: method_key.clone(),
            from: last.to_string().into(),
            call_site: CallSegment {
                address_filename: callee.address.source.filepath.clone(),
                address_line_col: callee
                    .address
                    .source
                    .location(callee.address.span.start)
                    .into(),
            },
        });
    }

    // Performs a function call.
    // Supported both native and regular functions.
    fn call_func(
        &mut self,
        realm: SharedRealm,
        callee_addr: Option<&Address>,
        func: &Value,
        args: &[Value],
    ) -> InterpreterResult<ControlFlow> {
        debug!("Call function with parameters {args:?}");

        if let Value::Native(native) = func {
            let new_realm = Realm::dive(realm);

            return native(self, Arc::new(RwLock::new(new_realm)), args);
        }

        if let Value::Function(func) = func {
            let mut new_realm = Realm::dive(Arc::clone(&func.closure_realm));

            let parameters = &func.params;

            if parameters.len() != args.len() {
                return Err(InterpreterError::CallError(
                    CallError::InsufficentArguments {
                        callee_address: callee_addr.unwrap().clone(),
                        expected_count: parameters.len(),
                        given_count: args.len(),
                    },
                ));
            }

            // Arguments are just temporary variables

            for (par, arg) in parameters.iter().zip(args) {
                new_realm.values_mut().insert(par.clone(), arg.clone());
            }

            let result =
                self.exec_single_statement(Arc::new(RwLock::new(new_realm)), &func.body)?;

            debug!(
                "Executing func with params {:?} returned {:?}",
                func.params, result
            );

            return Ok(match result {
                ControlFlow::Return(v) => ControlFlow::Value(v),
                other => other,
            });
        }

        Ok(ControlFlow::Nothing)
    }

    pub fn call_func_extern(
        &mut self,
        name: &str,
        args: &[Value],
    ) -> InterpreterResult<Option<ControlFlow>> {
        let method = match self.world.read().unwrap().lookup(name) {
            Some(v) => v,
            None => return Ok(None),
        };

        if let Value::Native(native) = method {
            let new_realm = Realm::dive(Arc::clone(&self.world));

            return Ok(Some(native(self, Arc::new(RwLock::new(new_realm)), args)?));
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

            let result =
                self.exec_single_statement(Arc::new(RwLock::new(new_realm)), &func.body)?;

            return Ok(Some(match result {
                ControlFlow::Return(v) => ControlFlow::Value(v),
                other => other,
            }));
        }

        Ok(None)
    }

    fn binary_op_helper(
        &mut self,
        realm: SharedRealm,
        op: &str,
        lhs: &Expression,
        rhs: &Expression,
    ) -> InterpreterResult<Option<Value>> {
        let lhs_val = self.evaluate_expression(Arc::clone(&realm), lhs, true)?;
        let rhs_val = self.evaluate_expression(Arc::clone(&realm), rhs, true)?;

        let ControlFlow::Value(lhs_val) = lhs_val else {
            panic!("A value should be returned by LHS, got: {lhs_val:?}");
        };

        let ControlFlow::Value(rhs_val) = rhs_val else {
            panic!("A value should be returned by RHS, got: {rhs_val:?}");
        };

        self.binary_op_helper_values(
            realm,
            op,
            Spanned::new(lhs_val, lhs.address.clone()),
            Spanned::new(rhs_val, rhs.address.clone()),
        )
    }

    fn binary_op_helper_values(
        &mut self,
        realm: SharedRealm,
        op: &str,
        lhs: Spanned<Value>,
        rhs: Spanned<Value>,
    ) -> InterpreterResult<Option<Value>> {
        let l_type = types::value_to_internal_type(&lhs.value).unwrap();
        let r_type = types::value_to_internal_type(&rhs.value).unwrap();

        let method_name = format!("operator{op}{r_type}");

        let method = realm
            .read()
            .unwrap()
            .lookup(&l_type)
            .and_then(|x| x.as_module())
            .map(|x| x.realm.read().unwrap().lookup(&method_name))
            .flatten()
            .ok_or_else(|| {
            InterpreterError::IncompatibleTypesForBinaryOperation {
                op: op.to_string(),
                lhs_addr: lhs.address.clone(),
                rhs_addr: rhs.address.clone(),
                lhs_type: l_type.to_string(),
                rhs_type: r_type.to_string(),
            }
        })?;

        if let ControlFlow::Value(va) =
            self.call_func(realm, None, &method, &[lhs.value, rhs.value])?
        {
            Ok(Some(va))
        } else {
            panic!("Failed to get a return value from function call.");
        }
    }

    fn unary_op_helper(
        &mut self,
        realm: SharedRealm,
        op: &str,
        expr: &Expression,
    ) -> InterpreterResult<Option<Value>> {
        let expr_val = self.evaluate_expression(Arc::clone(&realm), expr, true)?;

        let ControlFlow::Value(expr_val) = expr_val else {
            panic!("A value should be returned, got: {expr_val:?}");
        };

        let ty = types::value_to_internal_type(&expr_val).unwrap();

        let method_name = format!("operator{op}");

        let method = realm
            .read()
            .unwrap()
            .lookup(&ty)
            .and_then(|x| x.as_module())
            .map(|x| x.realm.read().unwrap().lookup(&method_name))
            .flatten()
            .ok_or_else(|| {
            InterpreterError::IncompatibleTypesForUnaryOperation {
                op: op.to_string(),
                addr: expr.address.clone(),
                ty: ty.to_string(),
            }
        })?;

        if let ControlFlow::Value(va) = self.call_func(realm, None, &method, &[expr_val])? {
            Ok(Some(va))
        } else {
            panic!("Failed to get a return value from function call.");
        }
    }

    fn resolve_lvalue(
        &mut self,
        realm: SharedRealm,
        expr: &Expression,
    ) -> InterpreterResult<LValue> {
        match &expr.value {
            ExprKind::Identifier(name) => Ok(LValue::Identifier(name.clone())),

            ExprKind::IndexedAccess { origin, index } => {
                let container = self.evaluate_expression(Arc::clone(&realm), origin, true)?;
                let index = self.evaluate_expression(Arc::clone(&realm), index, true)?;

                let ControlFlow::Value(container) = container else {
                    panic!("Expected value as container, got: {container:?}");
                };

                let ControlFlow::Value(index) = index else {
                    panic!("Expected value as index, got: {index:?}");
                };

                Ok(LValue::Index { container, index })
            }

            ExprKind::PropertyAccess { origin, property } => {
                let object = self.evaluate_expression(Arc::clone(&realm), origin, true);
                let name = property.value.as_id().unwrap().to_owned();

                let Ok(ControlFlow::Value(object)) = object else {
                    panic!("Expected value as object, got: {object:?}");
                };

                Ok(LValue::Property { object, name })
            }

            ExprKind::Path { .. } => {
                let stems = self.path_segments_to_vec(expr);
                let (name, path) = stems.split_last().unwrap();

                let mut node: Option<Value> = None;

                for (idx, i) in path.iter().enumerate() {
                    if let Some(v) = realm.read().unwrap().lookup(i) {
                        node = Some(v);
                    } else {
                        return Err(InterpreterError::NameNotDefined {
                            name: stems[0..=idx].join("::"),
                            address: expr.address.clone(),
                        });
                    }
                }

                // todo!("Resolved {path:?} into {node:?}")

                Ok(LValue::Property {
                    object: node.unwrap(),
                    name: name.clone(),
                })
            }
            _ => panic!("Invalid target ({:?})", expr.value),
        }
    }

    fn read_lvalue(&mut self, realm: SharedRealm, target: &LValue) -> Value {
        match target {
            LValue::Identifier(name) => realm.read().unwrap().lookup(name.as_str()).unwrap(),
            LValue::PrivateIdentifier(name) => realm.read().unwrap().lookup(name.as_str()).unwrap(),
            LValue::Index { container, index } => {
                let Value::Array(arr) = container else {
                    panic!("Cannot index into type: {:?}", container);
                };

                let Value::Integer(i) = index else {
                    panic!("Type `{:?}` cannot be used as an index", index);
                };

                arr.lock().unwrap()[*i as usize].clone()
            }
            LValue::Property { object, name } => {
                if let Value::RecordInstance(object) = object {
                    return object.read().unwrap().lookup(name).unwrap().clone();
                }

                if let Value::Module(mo) = object {
                    return mo.realm.read().unwrap().lookup(name).unwrap().clone();
                }

                panic!("Unexpected property object type: {object:?}")
            }
        }
    }

    // I separated it into a function to make it compatible with OpAssignments (+=, -=, ...)
    fn assign(&mut self, realm: SharedRealm, target: LValue, value: Value) {
        debug!("!!! Assignment: {target:?}");

        match target {
            LValue::Identifier(name) => {
                let assigned = realm.write().unwrap().write_existing(&name, value.clone());

                if !assigned {
                    // If it doesn't exist, create it locally
                    realm.write().unwrap().values_mut().insert(name, value);
                }
            }
            LValue::PrivateIdentifier(name) => {
                // Always create it locally
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
                let arc_bind = object
                    .as_record_instance()
                    .unwrap_or_else(|| panic!("Expected record instance, got: {object:?}"));

                let mut bind = arc_bind.write().unwrap();

                let val_bind = bind
                    .fields
                    .iter_mut()
                    .find(|fi| fi.name == name)
                    .map(|x| &mut x.value)
                    .unwrap_or_else(|| {
                        panic!("Can't find a field `{name}`");
                    });

                *val_bind = value;

                drop(bind);
            }
        }
    }

    fn compound_assignment_helper(
        &mut self,
        realm: SharedRealm,
        op: &str,
        name: &Expression,
        value: &Expression,
        is_subexpression: bool,
    ) -> InterpreterResult<ControlFlow> {
        let target = self.resolve_lvalue(Arc::clone(&realm), name)?;

        // Read current value — identifier needs a lookup, indexed needs evaluation
        let current = self.read_lvalue(Arc::clone(&realm), &target);
        let rhs = self.evaluate_expression(Arc::clone(&realm), value, true)?;

        let ControlFlow::Value(rhs) = rhs else {
            panic!("Expected RHS as value, got: {rhs:?}");
        };

        let result = self
            .binary_op_helper_values(
                Arc::clone(&realm),
                op,
                Spanned::new(current, name.address.clone()),
                Spanned::new(rhs, value.address.clone()),
            )?
            .unwrap();

        self.assign(Arc::clone(&realm), target, result.clone());

        Ok(if is_subexpression {
            ControlFlow::Value(result)
        } else {
            ControlFlow::Nothing
        })
    }

    fn path_segments_to_vec(&self, expr: &Expression) -> Vec<String> {
        match &expr.value {
            ExprKind::Path { parent, value } => {
                let lhs = self.path_segments_to_vec(parent);
                let rhs = self.path_segments_to_vec(value);

                lhs.into_iter().chain(rhs).collect::<Vec<String>>()
            }
            ExprKind::Identifier(name) => vec![name.clone()],
            _ => panic!("Invalid path segment: {:?}", expr.value),
        }
    }

    /// Flattens path into a string that will be used in Realm's HashMap lookup.
    fn flatten_path(&self, expr: &Expression) -> String {
        self.path_segments_to_vec(expr).join("::")
    }

    /// "Forks" an interpreter.
    /// It shares world, builtins and module registry, but creates new call trace stack.
    /// A forked interpreter can be safely put into a new thread like `thread::spawn(move || /* There's a forked interpreter */)`.
    /// TODO: It's planned for multithreading. May be changed, or completely removed.
    pub fn fork(&self) -> Self {
        Self {
            world: Arc::clone(&self.world),
            builtins: Arc::clone(&self.builtins),
            module_registry: Arc::clone(&self.module_registry),
            call_trace: LinkedList::new(),
        }
    }
}
