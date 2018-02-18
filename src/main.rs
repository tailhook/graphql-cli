extern crate graphql_parser;
extern crate failure;
#[macro_use] extern crate structopt;

use std::process::exit;

use structopt::StructOpt;

mod format;

#[derive(StructOpt)]
#[structopt(name = "graphql",
            about = "graphql command-line manipulation tool")]
enum Options {
    #[structopt(name = "format-query",
        about="format piece of query definition language")]
    FormatQuery(format::Options),
    #[structopt(name = "fq", about="alias for format-query")]
    FormatQueryShort(format::Options),
    #[structopt(name = "format-schema",
        about="format piece of schema definition language")]
    FormatSchema(format::Options),
    #[structopt(name = "fs", about="alias for format-schema")]
    FormatSchemaShort(format::Options),
}

fn main() {
    let result = match Options::from_args() {
        Options::FormatQuery(fq) | Options::FormatQueryShort(fq) => {
            format::query(fq)
        }
        Options::FormatSchema(fs) | Options::FormatSchemaShort(fs) => {
            format::schema(fs)
        }
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
