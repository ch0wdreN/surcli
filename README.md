# surcli

surrealDBにコマンドラインから触るためのCLIツール的なのを適当に実装

```
Usage: surcli [OPTIONS] --namespace <TEXT> --dbname <TEXT>

Options:
  -u, --user <TEXT>       Username to connect to the SurrealDB [default: root]
      --host <TEXT>       Host address of the SurrealDB [default: localhost:8000]
  -p, --password <TEXT>   Password to connect of the SurrealDB [default: root]
  -n, --namespace <TEXT>  Specify the SurrealDB namespace you want to connect to
  -d, --dbname <TEXT>     Specify the SurrealDB database name you want to connect to
  -f, --file <PATH>       Specify the path of the sql file you want to pass to SurrealDB
  -h, --help              Print help information
  -V, --version           Print version information
  ```