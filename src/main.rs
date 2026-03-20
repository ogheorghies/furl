use furl::format;
use furl::json;

use argh::FromArgs;
use std::fmt;
use std::io::{self, Write};
use url::Url;

#[derive(FromArgs)]
#[argh(
    description = r#"Text (printf) and JSON formatter for URLs.
Percent-encoded components are decoded by default. Use -e to keep them encoded.

Format specifiers (-f):
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

Unknown specifiers are printed as such."#,
    example = r#"% {command_name} 'postgres://usr:pwd@localhost:5432/db' \
       -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'"
host='localhost' port='5432' db='db' user='usr' pwd='pwd'

% {command_name} 'postgres://usr:pwd@localhost:5432/db'
postgres localhost 5432 db usr pwd

% {command_name} -j -p 'https://www.example.com/'
{{"scheme":"https","host":"www.example.com","port":443}}

% {command_name} -j 'https://usr:pwd@www.example.com/at?a=A&b=B#foo'
{{"scheme":"https","user":"usr","password":"pwd","host":"www.example.com","path":"/at","query":{{"a":"A","b":"B"}},"fragment":"foo"}}"#
)]
struct Args {
    /// the format to use [default: "%s %h %p %A %U %P %q %f"]
    #[argh(option, short = 'f')]
    format: Option<String>,

    /// output as JSON
    #[argh(switch, short = 'j')]
    json: bool,

    /// include default port in JSON output (e.g. 80 for http)
    #[argh(switch, short = 'p')]
    default_port: bool,

    /// output percent-encoded URLs without decoding
    #[argh(switch, short = 'e')]
    encoded: bool,

    /// the URL to parse and format
    #[argh(option, short = 'u')]
    url: Option<String>,

    /// the URL to parse and format (positional alternative to -u)
    #[argh(positional)]
    positional_url: Option<String>,
}

enum AppErr {
    UrlParseError(url::ParseError),
    IoError(io::Error),
    MissingUrl,
}

impl fmt::Debug for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::UrlParseError(err) => write!(f, "invalid URL: {err}"),
            AppErr::IoError(err) => write!(f, "I/O error: {err}"),
            AppErr::MissingUrl => write!(f, "missing URL: provide as positional argument or with -u"),
        }
    }
}

impl From<url::ParseError> for AppErr {
    fn from(err: url::ParseError) -> Self {
        AppErr::UrlParseError(err)
    }
}

impl From<io::Error> for AppErr {
    fn from(err: io::Error) -> Self {
        AppErr::IoError(err)
    }
}

fn main() -> Result<(), AppErr> {
    let args: Args = argh::from_env();
    let raw_url = args.url.or(args.positional_url).ok_or(AppErr::MissingUrl)?;
    let url = Url::parse(raw_url.as_str())?;

    let decode = !args.encoded;
    let mut out = io::stdout().lock();
    let result = if args.json {
        writeln!(out, "{}", json::UrlParts::from_url(&url, args.default_port, decode))
    } else {
        let fmt = args.format.as_deref().unwrap_or("%s %h %p %A %U %P %q %f");
        writeln!(out, "{}", format::format_url(fmt, &url, decode))
    };

    match result {
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        other => Ok(other?),
    }
}
