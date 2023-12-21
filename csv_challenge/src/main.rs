mod err;
mod read;
mod write;

use std::path::PathBuf;
use std::process;
use clap::Parser;
use crate::read::{load_csv, write_csv};
use crate::write::replace_column;

/// 命令行调用:
/// /home/justin/.cargo/bin/cargo run --color=always --bin csv_challenge --manifest-path /home/justin/workspace/rust/rust-get-started/csv_challenge/Cargo.toml
/// -- csv_challenge/input/challenge.csv City Beijing csv_challenge/output/output.csv
fn main() {
    let cli = Cli::parse();
    let filename = PathBuf::from(cli.input);
    let csv_content = match load_csv(filename) {
        Ok(content) => { content },
        Err(e) => {
            println!("read error: {:?}", e);
            process::exit(1);
        }
    };
    let modified_data = match replace_column(csv_content, &cli.column_name, &cli.replacement) {
        Ok(content) => content,
        Err(e) => {
            println!("replace error: {:?}", e);
            process::exit(1);
        }
    };
    let output_file = cli.output.unwrap_or("csv_challenge/output/output.csv".to_string());
    match write_csv(&modified_data, &output_file) {
        Ok(_) => println!("write success!"),
        Err(e) => {
            println!("write error: {:?}", e);
            process::exit(1);
        }
    }
}

// 利用clap处理命令行参数
#[derive(Debug, Parser)]
struct Cli {
    input: String,
    column_name: String,
    replacement: String,
    output: Option<String>
}
