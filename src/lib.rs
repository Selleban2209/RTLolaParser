use std::{ffi::CStr, os::raw::c_char, path::PathBuf};
use rtlola_frontend::{parse, parse_to_ast, ParserConfig};
use ast::RtLolaAst;
use rtlola_parser::ast::{self, BinOp, Expression, ExpressionKind, LitKind};
use rtlola_frontend::mir::RtLolaMir;
pub use rtlola_parser::ast::OutputKind;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::fmt::Debug;

#[derive(Serialize)]
struct SpecificationJson {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    triggers: Vec<Trigger>,
}

#[derive(Serialize)]
struct Input {
    name: String,
    type_: String,
}


#[derive(Serialize)]
struct Output {
    variable: String,
    comparison: String,
}

#[derive(Serialize)]
struct Trigger {
    condition: String,
    message: String,
}

#[no_mangle]
pub extern "C" fn parse_specification(file_path: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!file_path.is_null());
        CStr::from_ptr(file_path)
    };
    let file_path = c_str.to_str().unwrap();
    println!("Parsing specification from file: {}", file_path);
    let file_path = PathBuf::from(file_path);

    let config = match ParserConfig::from_path(file_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load specification: {}", err);
            return std::ptr::null_mut();
        }
    };

    let ast = match config.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Failed to parse specification: {:?}", err);
            return std::ptr::null_mut();
        }
    };
    //println!("Parsed AST: {:?}", ast);

    let json_data = rtlola_ast_to_json(&ast);
    let json_string = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON");
    //print!("JSON: {}", json_string);
    let c_string = std::ffi::CString::new(json_string).unwrap();
    c_string.into_raw()
    
}

#[no_mangle]
pub extern "C" fn free_json_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        let _ = std::ffi::CString::from_raw(s);
    }
}

fn rtlola_ast_to_json(ast: &RtLolaAst) -> SpecificationJson {
    let inputs: Vec<Input> = ast
        .inputs
        .iter()
        .map(|input| Input {
            name: input.name.name.clone(),
            type_: input.ty.to_string(),
        })
        .collect();

    let outputs: Vec<Output> = ast
        .outputs
        .iter()
        .filter_map(|output| match &output.kind {
            OutputKind::NamedOutput(name) => {
                if let Some(eval_spec) = output.eval.first() {
                    if let Some(Expression {
                        kind: ExpressionKind::Binary(op, lhs, rhs),
                        ..
                    }) = &eval_spec.eval_expression
                    {
                        Some(Output {
                            variable: name.name.clone(),
                            comparison: format!(
                                "{} {} {}",
                                expression_to_string(lhs),
                                operator_to_string(op),
                                expression_to_string(rhs)
                            ),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let triggers: Vec<Trigger> = ast
        .outputs
        .iter()
        .filter_map(|output| match &output.kind {
            OutputKind::Trigger => {
                let condition = output
                    .eval
                    .first()
                    .and_then(|eval| eval.condition.as_ref())
                    .map(|expr| expression_to_string(expr))
                    .unwrap_or_else(|| "No condition".to_string());

                let message = output
                    .eval
                    .first()
                    .and_then(|eval| {
                        eval.eval_expression
                            .as_ref()
                            .map(|expr| expression_to_string(expr))
                    })
                    .unwrap_or_else(|| "No message".to_string());

                Some(Trigger { condition, message })
            }
            _ => None,
        })
        .collect();

    SpecificationJson {
        inputs,
        outputs,
        triggers,
    }
}

fn operator_to_string(op: &BinOp) -> &str {
    match op {
        BinOp::Gt => ">",
        BinOp::Lt => "<",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Ge => ">=",
        BinOp::Le => "<=",
        _ => "unknown",
    }
}

fn expression_to_string(expr: &Expression) -> String {
    match &expr.kind {
        ExpressionKind::Ident(ident) => ident.name.clone(),
        ExpressionKind::Lit(literal) => match &literal.kind {
            LitKind::Numeric(value, _) => value.clone(),
            LitKind::Str(value) => value.clone(),
            LitKind::RawStr(_) => todo!(),
            LitKind::Bool(_) => todo!(),
        },
        ExpressionKind::Binary(op, lhs, rhs) => format!(
            "{} {} {}",
            expression_to_string(lhs),
            operator_to_string(op),
            expression_to_string(rhs)
        ),
        _ => "complex_expression".to_string(),
    }
}

