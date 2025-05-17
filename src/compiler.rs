use crate::ast::{Struct, Type};
use crate::cli::Cli;
use crate::parser::Parser;
use serde_json::json;
use std::fs;
use std::path::Path;

pub fn compile(cli: &Cli) {
    let source = fs::read_to_string(&cli.input_file).expect("Failed to read input file");

    let mut parser = Parser::new(&source);
    let structs = parser.parse();

    if cli.json {
        let json_output = generate_json_output(&structs);

        if cli.pwd {
            let out_path = Path::new(&cli.input_file).with_extension("json");
            fs::write(&out_path, &json_output).expect("Failed to write output");
            println!("Wrote JSON to {}", out_path.display());
        } else {
            println!("{}", json_output);
        }
    }
}

fn generate_json_output(structs: &[Struct]) -> String {
    let Some(root) = structs.last() else {
        return "{}".into();
    };

    let map = build_placeholder(&root, structs);
    serde_json::to_string_pretty(&map).unwrap()
}

fn build_placeholder(root: &Struct, structs: &[Struct]) -> serde_json::Value {
    let mut obj = serde_json::Map::new();

    for field in &root.fields {
        let value = match &field.ty {
            Type::String => json!("example"),
            Type::Int => json!(0),
            Type::Bool => json!(true),
            Type::Array(inner) => {
                let inner_value = match &**inner {
                    Type::Custom(name) => {
                        if let Some(s) = structs.iter().find(|s| &s.name == name) {
                            build_placeholder(s, structs)
                        } else {
                            json!({})
                        }
                    }
                    base => build_primitive(base),
                };
                json!([inner_value])
            }
            Type::Custom(name) => {
                if let Some(s) = structs.iter().find(|s| &s.name == name) {
                    build_placeholder(s, structs)
                } else {
                    json!({})
                }
            }
        };

        obj.insert(field.name.clone(), value);
    }

    serde_json::Value::Object(obj)
}

fn build_primitive(ty: &Type) -> serde_json::Value {
    match ty {
        Type::String => json!("example"),
        Type::Int => json!(0),
        Type::Bool => json!(true),
        _ => json!(null),
    }
}
