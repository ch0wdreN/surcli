use clap::Parser;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
pub struct Cli {
    /// Username to connect to the SurrealDB
    #[clap(short, long, value_name = "TEXT", default_value = "root")]
    pub user: String,

    /// Host address of the SurrealDB
    #[clap(long, value_name = "TEXT", default_value = "localhost:8000")]
    pub host: String,

    /// Password to connect of the SurrealDB
    #[clap(short, long, value_name = "TEXT", default_value = "root")]
    pub password: String,

    /// Specify the SurrealDB namespace you want to connect to
    #[clap(short, long, value_name = "TEXT")]
    pub namespace: String,

    /// Specify the SurrealDB database name you want to connect to
    #[clap(short, long, value_name = "TEXT")]
    pub dbname: String,

    /// Specify the path of the sql file you want to pass to SurrealDB
    #[clap(short, long, value_name = "PATH")]
    pub file: Option<String>,
}
