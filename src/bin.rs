use clap::*;
use log::*;
use sane_csv_json_lib::*;
use serde_json::{json, Value};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CliArgs {
    /// input path of the csv
    input_path: String,

    #[clap(short, long)]
    /// path of the config json
    config_path: String,

    #[clap(short, long)]
    /// desired output path of the config json
    out_path: Option<String>,
}

fn main() {
    env_logger::init();

    let args = CliArgs::parse();

    let out = InputService::handle_input(InputArgs {
        csv_path: &args.input_path,
        out_path: args.out_path.as_deref(),
        config_path: Some(&args.config_path),
    });

    if let Err(err) = out {
        error!("{}", err);
        return;
    }

    let out = out.unwrap();
    let mut arr = Vec::<Value>::new();
    for n in out.records.into_iter() {
        let value = ParseService::parse_row(&out.schema, n);
        arr.push(value)
    }

    let output = json!({ "data": arr }).to_string();
    fs::write("data.json", output).unwrap();
}
