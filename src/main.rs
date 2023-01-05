use clap::Parser;
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::path::Path;
use surcli::cli::Cli;

fn construct_headers(ns: &str, db: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert("NS", HeaderValue::from_str(ns).unwrap());
    headers.insert("DB", HeaderValue::from_str(db).unwrap());
    headers
}

fn get_file_data(arg_file: String) -> String {
    let path = Path::new(&arg_file);
    let mut file = File::open(path).unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();
    body
}

fn fetch(req_url: String, headers: HeaderMap, body: String) -> Result<String, reqwest::Error> {
    let client = blocking::Client::new();
    let res = client
        .post(req_url)
        .headers(headers)
        .body(body)
        .send()?
        .text()?;
    Ok(res)
}

fn main() {
    let cli = Cli::parse();

    let url = cli.host;
    let user = cli.user;
    let password = cli.password;
    let ns = cli.namespace;
    let db = cli.dbname;
    let req_url = format!("http://{}:{}@{}/sql", user, password, url);
    println!("Connect to {}", req_url);
    println!("namespace: {} db: {}", ns, db);

    let arg_file = cli.file;
    let headers = construct_headers(ns.as_str(), db.as_str());

    if let Some(f) = arg_file {
        let body = get_file_data(f);
        let res = fetch(req_url.clone(), headers.clone(), body.clone()).unwrap();
        println!("{}", res);
        std::process::exit(0);
    }

    loop {
        print!("surreal:{}> ", db);
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).expect("failed to read line");
        let res = fetch(req_url.clone(), headers.clone(), line).unwrap();
        println!("{}", res);
    }
}
