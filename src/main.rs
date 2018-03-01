extern crate serde;
extern crate serde_json;
extern crate graphql_parser;
extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt;

use std::process::exit;

use structopt::StructOpt;

mod format;
mod jsonize;

#[derive(StructOpt)]
#[structopt(name = "graphql",
            about = "graphql command-line manipulation tool")]
enum Options {
    #[structopt(name = "format-query", alias = "fq",
        about="format piece of query definition language")]
    FormatQuery(format::Options),
    #[structopt(name = "format-schema", alias = "fs",
        about="format piece of schema definition language")]
    FormatSchema(format::Options),
    #[structopt(name = "json-query",
        about="format a whole query as a json (to be used with e.g. curl)")]
    JsonizeQuery(jsonize::Options),
}

fn main() {
    let result = match Options::from_args() {
        Options::FormatQuery(fq) => format::query(fq),
        Options::FormatSchema(fs) => format::schema(fs),
        Options::JsonizeQuery(jo) => jsonize::jsonize(jo),
    };
    match result {
        Ok(()) => {
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
