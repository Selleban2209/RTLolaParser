
use std::{fs, path::PathBuf};
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

fn main(){
    // Specify the path to your specification file
    let file_path = PathBuf::from("resources/lola_spec.lola");

    // Load the configuration from the file
    let config = match ParserConfig::from_path(file_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load specification: {}", err);
            return;
        }
    };

    // Parse the specification
    let ast = match config.parse() {
    Ok(ast) => ast,
    Err(err) => {
        eprintln!("Failed to parse specification: {:?}", err);
        return;
        }   
    };


    println!("Parsed AST: {:?}", ast);
    let json_data = rtlola_ast_to_json(&ast);
    
    let output_path = "resources/RTLola_output.json";
    let mut file = File::create(output_path).expect("Failed to create JSON file");
    file.write_all(
        serde_json::to_string_pretty(&json_data)
            .expect("Failed to serialize JSON")
            .as_bytes(),
    )
    .expect("Failed to write JSON data");

    println!("Specification successfully written to {}", output_path);
}

fn rtlola_ast_to_json(ast: &RtLolaAst) -> SpecificationJson{

  let inputs: Vec<Input> = ast
        .inputs
        .iter()
        .map(|input| Input {
            name: input.name.name.clone(),
            type_: input.ty.to_string(), // Assuming `ty` is the field representing the type
            })
            .collect();

            // Updated code for constructing outputs
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
                                expression_to_string(lhs), // Converts the left-hand side to a string
                                operator_to_string(op),    // Converts the operator to a string
                                expression_to_string(rhs)  // Converts the right-hand side to a string
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
                .map(|expr| expression_to_string(expr)) // Convert condition to string
                .unwrap_or_else(|| "No condition".to_string());

            let message = output
                .eval
                .first()
                .and_then(|eval| {
                    eval.eval_expression
                        .as_ref()
                        .map(|expr| expression_to_string(expr)) // Convert message to string
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

// Helper function to convert an expression to a string
fn expression_to_string(expr: &Expression) -> String {
    match &expr.kind {
        ExpressionKind::Ident(ident) => ident.name.clone(), // Variable names directly
        ExpressionKind::Lit(literal) => match &literal.kind {
            LitKind::Numeric(value, _) => value.clone(), // Numeric values
            LitKind::Str(value) => value.clone(),
            //Todo -> Extend the literal structs
            LitKind::RawStr(_) => todo!(),
            LitKind::Bool(_) => todo!(),        // String values
        },
        ExpressionKind::Binary(op, lhs, rhs) => format!(
            "{} {} {}",
            expression_to_string(lhs),       // Left-hand side as string
            operator_to_string(op),          // Operator as string
            expression_to_string(rhs)        // Right-hand side as string
        ),
        _ => "complex_expression".to_string(), // Fallback for unsupported kinds
    }
}