Command line formatter for URLs, written in Rust, based on [url](https://docs.rs/url)
and [clap](https://docs.rs/clap).

Install with: `cargo install furl`

Examples:

```bash
$ furl -u "postgres://usr:pwd@localhost:5432/db" -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'"
host='localhost' port='5432' db='db' user='usr' pwd='pwd'

$ furl -u "https://www.google.com/search?q=rust+furl" -f "scheme='%s' query='%q' path='%a'"
scheme='https' query='q=rust+furl' path='/search'

$ furl -u "https://en.wikipedia.org/wiki/Rust#Prevention" -f "port='%p' fragment='%f'"
port='' fragment='Prevention'

$ furl -u "postgres://usr:pwd@localhost:5432/db"
postgres localhost 5432 db usr pwd  
```

The formatting string can contain any of the following substitutions:
```text
%A - the path, without the starting '/'{n}
%a - the path{n}
%f - the fragment{n}
%h - the hostname{n}
%P - the password of the userinfo portion{n}
%p - the port{n}
%q - the query string{n}
%s - the scheme{n}
%U - the username of the userinfo portion{n}
%n - newline (\n)
%t - tab (\t)
%% - a single %
```

# Bash example

```bash
echo 'DATABASE_URL="postgres://webapp:pwd@localhost:5432/myapp"' >> .env

DPG="docker run -d --rm --name pg-%A -v vol-%A:/var/lib/postgresql -p %p:5432
    -e POSTGRES_DB=%A -e POSTGRES_USER=%U -e POSTGRES_PASSWORD=%P postgres"

$(source .env && furl -u $DATABASE_URL -f "$DPG")
```
