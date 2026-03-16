Text (printf) and JSON formatter for URLs.

Install with: `cargo install furl`

Can also be used as a library: `cargo add furl`

Examples:

```bash
$ furl -u "https://www.example.com/" -j -p
{"scheme":"https","host":"www.example.com","port":443}

$ furl -u "https://usr:pwd@www.example.com/at?a=A&b=B#foo" -j
{"scheme":"https","user":"usr","password":"pwd","host":"www.example.com","path":"/at","query":{"a":"A","b":"B"},"fragment":"foo"}

$ furl -u "postgres://usr:pwd@localhost:5432/db" \
       -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'"
host='localhost' port='5432' db='db' user='usr' pwd='pwd'

$ furl -u "https://www.google.com/search?q=rust+furl" \
       -f "scheme='%s' query='%q' path='%a' port='%p'"
scheme='https' query='q=rust+furl' path='/search' port=''

$ furl -u "https://en.wikipedia.org/wiki/Rust#Prevention" \
       -f "path='%a' fragment='%f'"
path='/wiki/Rust' fragment='Prevention'

$ furl -u "postgres://usr:pwd@localhost:5432/db"
postgres localhost 5432 db usr pwd  
```

# JSON output

Use `-j` to output URL components as JSON. Only detected components are included.
Query parameters are expanded into a nested object.

```bash
$ furl -u "https://usr:pwd@www.example.com/at?a=A&b=B#foo" -j | jq
{
  "scheme": "https",
  "user": "usr",
  "password": "pwd",
  "host": "www.example.com",
  "path": "/at",
  "query": {
    "a": "A",
    "b": "B"
  },
  "fragment": "foo"
}
```

Default ports (80 for http, 443 for https) are omitted unless `-p` is used:

```bash
$ furl -u "https://www.example.com:443/" -j
{"scheme":"https","host":"www.example.com"}

$ furl -u "https://www.example.com/" -j -p
{"scheme":"https","host":"www.example.com","port":443}
```

# Format string

The formatting string (`-f`) can contain any of the following substitutions:
```text
%A - the path, without the starting '/'
%a - the path
%f - the fragment
%h - the hostname
%P - the password of the userinfo portion
%p - the port
%q - the query string
%s - the scheme
%U - the username of the userinfo portion
%n - newline (\n)
%t - tab (\t)
%% - a single %
```

# Library usage

`furl` exposes a `UrlParts` struct for parsing URLs into typed components:

```rust
use furl::json::UrlParts;
use url::Url;

let url = Url::parse("https://usr:pwd@example.com/path?a=A&b=B#frag").unwrap();
let parts = UrlParts::from_url(&url, false);

assert_eq!(parts.scheme, "https");
assert_eq!(parts.host, Some("example.com"));
assert_eq!(parts.user, Some("usr"));
assert_eq!(parts.password, Some("pwd"));
assert_eq!(parts.path, Some("/path"));
assert_eq!(parts.fragment, Some("frag"));

// Query params preserve URL order
assert_eq!(parts.query, Some(vec![("a", "A"), ("b", "B")]));

// Display impl outputs JSON
println!("{parts}");
// {"scheme":"https","user":"usr","password":"pwd","host":"example.com","path":"/path","query":{"a":"A","b":"B"},"fragment":"frag"}
```

`format_url` formats a URL using printf-style specifiers:

```rust
use furl::format::format_url;
use url::Url;

let url = Url::parse("postgres://usr:pwd@localhost:5432/db").unwrap();
let out = format_url("host='%h' port='%p' db='%A'", &url);
assert_eq!(out, "host='localhost' port='5432' db='db'");
```

# Bash example

```bash
echo 'DATABASE_URL="postgres://webapp:pwd@localhost:5432/myapp"' >> .env

DPG="docker run -d --rm --name pg-%A -v vol-%A:/var/lib/postgresql \
    -p %p:5432 \
    -e POSTGRES_DB=%A -e POSTGRES_USER=%U -e POSTGRES_PASSWORD=%P \
    postgres"

$(source .env && furl -u $DATABASE_URL -f "$DPG")
```
