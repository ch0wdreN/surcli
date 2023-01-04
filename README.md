# surcli

surrealDBにコマンドラインから触るためのCLIツール
現在はファイルをもらっている状態だからdbcli的な感じで対話環境を実装したい

```
Usage: surcli [OPTIONS] --namespace <TEXT> --dbname <TEXT> --file <FILE>

Options:
  -u, --user <TEXT>       Username to connect to the SurrealDB [default: root]
      --host <TEXT>       Host address of the SurrealDB [default: localhost:8000]
  -p, --password <TEXT>   Password to connect to the SurrealDB [default: root]
  -n, --namespace <TEXT>
  -d, --dbname <TEXT>
  -f, --file <FILE>
  -h, --help              Print help information
  -V, --version           Print version information
  ```
