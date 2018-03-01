use std::fs::File;
use std::io::{Write, stdin, stdout};
use std::path::{PathBuf, Path};

use failure::Error;
use graphql_parser::{query};
use graphql_parser::{Style};
use format::read_ast_query;

use serde_json::{self, Value as Json, to_writer};


#[derive(StructOpt)]
pub struct Options {
    #[structopt(short="f", long="file", parse(from_os_str))]
    #[structopt(help="input file name (default stdin)")]
    input: Option<PathBuf>,
    #[structopt(short="o", long="output", parse(from_os_str))]
    #[structopt(help="output file name (default stdout)")]
    output: Option<PathBuf>,
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct GraphqlQuery {
    query: String,
    #[serde(skip_serializing_if="serde_json::Map::is_empty")]
    variables: serde_json::Map<String, Json>,
    #[serde(skip_serializing_if="Option::is_none")]
    operation_name: Option<String>,
}

pub fn format_graphql<F: Write>(f: F, opts: &Options, ast: query::Document)
    -> Result<(), Error>
{
    let query = ast.format(&Style::default());
    to_writer(f, &GraphqlQuery {
        query, variables: serde_json::Map::new(), operation_name: None,
    })?;
    Ok(())
}

pub fn jsonize(opts: Options) -> Result<(), Error> {
    let ast = match opts.input {
        Some(ref inp) if inp == Path::new("-") => {
            read_ast_query(stdin())?
        }
        Some(ref inp) => {
            read_ast_query(File::open(inp)?)?
        }
        None => {
            read_ast_query(stdin())?
        }
    };
    match opts.output {
        Some(ref outp) if outp == Path::new("-") => {
            format_graphql(stdout(), &opts, ast)?
        }
        Some(ref outp) => {
            format_graphql(File::create(outp)?, &opts, ast)?
        }
        None => {
            format_graphql(stdout(), &opts, ast)?
        }
    }
    Ok(())
}
