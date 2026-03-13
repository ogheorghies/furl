mod format;
mod json;

use argh::FromArgs;
use std::fmt;
use url::Url;

#[derive(FromArgs)]
#[argh(
    description = r#"Text (printf) and JSON formatter for URLs.

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
    example = r#"% {command_name} -u 'postgres://usr:pwd@localhost:5432/db' \
       -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'"
host='localhost' port='5432' db='db' user='usr' pwd='pwd'

% {command_name} -u 'postgres://usr:pwd@localhost:5432/db'
postgres localhost 5432 db usr pwd

% {command_name} -u 'https://www.example.com/' -j -p
{{"scheme":"https","host":"www.example.com","port":443}}

% {command_name} -u 'https://usr:pwd@www.example.com/at?a=A&b=B#foo' -j
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

    /// the URL to parse and format
    #[argh(option, short = 'u')]
    url: String,
}

enum AppErr {
    UrlParseError(url::ParseError),
}

impl fmt::Debug for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::UrlParseError(err) => write!(f, "invalid URL: {err}"),
        }
    }
}

impl From<url::ParseError> for AppErr {
    fn from(err: url::ParseError) -> Self {
        AppErr::UrlParseError(err)
    }
}

fn main() -> Result<(), AppErr> {
    let args: Args = argh::from_env();
    let url = Url::parse(args.url.as_str())?;

    if args.json {
        println!("{}", json::json_url(&url, args.default_port));
    } else {
        let fmt = args.format.as_deref().unwrap_or("%s %h %p %A %U %P %q %f");
        println!("{}", format::format_url(fmt, &url));
    }
    Ok(())
}
