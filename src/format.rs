use std::fs::File;
use std::io::{Read, Write, stdin, stdout};
use std::path::{PathBuf, Path};

use failure::Error;
use graphql_parser::{query, parse_query};
use graphql_parser::{schema, parse_schema};
use graphql_parser::{Style};


#[derive(StructOpt)]
pub struct Options {
    #[structopt(short="f", long="file", parse(from_os_str))]
    #[structopt(help="input file name (default stdin)")]
    input: Option<PathBuf>,
    #[structopt(short="o", long="output", parse(from_os_str))]
    #[structopt(help="output file name (default stdout)")]
    output: Option<PathBuf>,
    #[structopt(long="indent", help="indentation level", default_value="2")]
    indent: u32,
}

pub fn read_ast_query<F: Read>(mut f: F) -> Result<query::Document, Error> {
    let mut buf = String::with_capacity(1024);
    f.read_to_string(&mut buf)?;
    Ok(parse_query(&buf)?)
}

pub fn format_ast_query<F: Write>(mut f: F, ast: query::Document, s: &Style)
    -> Result<(), Error>
{
    write!(f, "{}", ast.format(s))?;
    Ok(())
}

pub fn read_ast_schema<F: Read>(mut f: F) -> Result<schema::Document, Error> {
    let mut buf = String::with_capacity(1024);
    f.read_to_string(&mut buf)?;
    Ok(parse_schema(&buf)?)
}

pub fn format_ast_schema<F: Write>(mut f: F, ast: schema::Document, s: &Style)
    -> Result<(), Error>
{
    write!(f, "{}", ast.format(s))?;
    Ok(())
}

pub fn query(opts: Options) -> Result<(), Error> {
    let ast = match opts.input {
        Some(ref inp) if inp == Path::new("-") => {
            read_ast_query(stdin())?
        }
        Some(inp) => {
            read_ast_query(File::open(&inp)?)?
        }
        None => {
            read_ast_query(stdin())?
        }
    };
    let mut style = Style::default();
    style.indent(opts.indent);
    match opts.output {
        Some(ref outp) if outp == Path::new("-") => {
            format_ast_query(stdout(), ast, &style)?
        }
        Some(outp) => {
            format_ast_query(File::create(&outp)?, ast, &style)?
        }
        None => {
            format_ast_query(stdout(), ast, &style)?
        }
    }
    Ok(())
}

pub fn schema(opts: Options) -> Result<(), Error> {
    let ast = match opts.input {
        Some(ref inp) if inp == Path::new("-") => {
            read_ast_schema(stdin())?
        }
        Some(inp) => {
            read_ast_schema(File::open(&inp)?)?
        }
        None => {
            read_ast_schema(stdin())?
        }
    };
    let mut style = Style::default();
    style.indent(opts.indent);
    match opts.output {
        Some(ref outp) if outp == Path::new("-") => {
            format_ast_schema(stdout(), ast, &style)?
        }
        Some(outp) => {
            format_ast_schema(File::create(&outp)?, ast, &style)?
        }
        None => {
            format_ast_schema(stdout(), ast, &style)?
        }
    }
    Ok(())
}
