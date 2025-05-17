use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(name = "compile")]
pub struct Cli {
    pub input_file: String,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub pwd: bool,
}
