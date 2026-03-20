Text (printf) and JSON formatter for URLs.

Percent-encoded components are decoded by default. Use `-e` to keep them encoded.

Install with: `cargo install furl`

Can also be used as a library: `cargo add furl`

Examples:

```bash
$ furl -j -p "https://www.example.com/"
{"scheme":"https","host":"www.example.com","port":443}

$ furl -j "https://usr:pwd@www.example.com/at?a=A&b=B#foo"
{"scheme":"https","user":"usr","password":"pwd","host":"www.example.com","path":"/at","query":{"a":"A","b":"B"},"fragment":"foo"}

$ furl "postgres://usr:pwd@localhost:5432/db" \
       -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'"
host='localhost' port='5432' db='db' user='usr' pwd='pwd'

$ furl "https://www.google.com/search?q=rust+furl" \
       -f "scheme='%s' query='%q' path='%a' port='%p'"
scheme='https' query='q=rust+furl' path='/search' port=''

$ furl "https://en.wikipedia.org/wiki/Rust#Prevention" \
       -f "path='%a' fragment='%f'"
path='/wiki/Rust' fragment='Prevention'

$ furl "postgres://usr:pwd@localhost:5432/db"
postgres localhost 5432 db usr pwd

$ furl "https://example.com/caf%C3%A9?q=hello%20world" -f "path='%a' query='%q'"
path='/café' query='q=hello world'

$ furl -e "https://example.com/caf%C3%A9" -f "%a"
/caf%C3%A9
```

# JSON output

Use `-j` to output URL components as JSON. Only detected components are included.
Query parameters are expanded into a nested object.

```bash
$ furl -j "https://usr:pwd@www.example.com/at?a=A&b=B#foo" | jq
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
$ furl -j "https://www.example.com:443/"
{"scheme":"https","host":"www.example.com"}

$ furl -j -p "https://www.example.com/"
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
let parts = UrlParts::from_url(&url, false, true);

assert_eq!(parts.scheme, "https");
assert_eq!(parts.host.as_deref(), Some("example.com"));
assert_eq!(parts.user.as_deref(), Some("usr"));
assert_eq!(parts.password.as_deref(), Some("pwd"));
assert_eq!(parts.path.as_deref(), Some("/path"));
assert_eq!(parts.fragment.as_deref(), Some("frag"));

// Display impl outputs JSON
println!("{parts}");
// {"scheme":"https","user":"usr","password":"pwd","host":"example.com","path":"/path","query":{"a":"A","b":"B"},"fragment":"frag"}
```

`format_url` formats a URL using printf-style specifiers:

```rust
use furl::format::format_url;
use url::Url;

let url = Url::parse("postgres://usr:pwd@localhost:5432/db").unwrap();
let out = format_url("host='%h' port='%p' db='%A'", &url, true);
assert_eq!(out, "host='localhost' port='5432' db='db'");
```

# Bash example

```bash
echo 'DATABASE_URL="postgres://webapp:pwd@localhost:5432/myapp"' >> .env

DPG="docker run -d --rm --name pg-%A -v vol-%A:/var/lib/postgresql \
    -p %p:5432 \
    -e POSTGRES_DB=%A -e POSTGRES_USER=%U -e POSTGRES_PASSWORD=%P \
    postgres"

$(source .env && furl "$DATABASE_URL" -f "$DPG")
```
