use std::{fs, path::PathBuf};
use rtlola_frontend::{parse_to_ast, ParserConfig};
pub use ast::RtLolaAst;
use rtlola_parser::ast;

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
      match config.parse() {
          Ok(ast) => println!("Parsed AST: {:?}", ast),
          Err(err) => eprintln!("Parsing failed: {:?}", err),
      }
}