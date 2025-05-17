pub mod ast;
pub mod cli;
pub mod compiler;
pub mod lexer;
pub mod parser;

use parser::Parser;

fn main() {
    let source = r#"
        [List]
        struct MyItem {
            name: String,
            count: i32,
        }

        struct Config {
            items: MyItem[],
            enabled: bool,
        }
    "#;

    let mut parser = Parser::new(source);
    let structs = parser.parse();

    for s in structs {
        println!("{:#?}", s);
    }
}
