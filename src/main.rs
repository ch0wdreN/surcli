use clap::Parser;
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    /// Username to connect to the SurrealDB.
    #[clap(short, long, value_name = "TEXT", default_value = "root")]
    user: String,

    /// Host address of the SurrealDB.
    #[clap(long, value_name = "TEXT", default_value = "localhost:8000")]
    host: String,

    /// Password to connect to the SurrealDB.
    #[clap(short, long, value_name = "TEXT", default_value = "root")]
    password: String,

    #[clap(short = 'n', long = "namespace", value_name = "TEXT")]
    namespace: String,

    #[clap(short = 'd', long = "dbname", value_name = "TEXT")]
    dbname: String,

    #[clap(short, long)]
    file: String,
}

fn construct_headers(ns: &str, db: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert("NS", HeaderValue::from_str(ns).unwrap());
    headers.insert("DB", HeaderValue::from_str(db).unwrap());
    headers
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let url = cli.host;
    let user = cli.user;
    let password = cli.password;
    let ns = cli.namespace;
    let db = cli.dbname;
    let req_url = format!("http://{}:{}@{}/sql", user, password, url);
    println!("Connect to {}", req_url);
    println!("namespace: {} db: {}", ns, db);

    let path = Path::new(&cli.file);
    let mut file = File::open(path).unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();

    let client = blocking::Client::new();

    let res = client
        .post(req_url)
        .headers(construct_headers(ns.as_str(), db.as_str()))
        .body(body)
        .send()?
        .text()?;

    println!("{}", res);

    Ok(())
}
