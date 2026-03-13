use argh::FromArgs;
use std::fmt;
use url::Url;

#[derive(FromArgs)]
#[argh(
    description = "Formatter for URLs using a sprintf-like template.\n\n\
    Format specifiers:\n\
    \x20 %A - the path, without the starting '/'\n\
    \x20 %a - the path\n\
    \x20 %f - the fragment\n\
    \x20 %h - the hostname\n\
    \x20 %P - the password of the userinfo portion\n\
    \x20 %p - the port\n\
    \x20 %q - the query string\n\
    \x20 %s - the scheme\n\
    \x20 %U - the username of the userinfo portion\n\
    \x20 %n - newline (\\n)\n\
    \x20 %t - tab (\\t)\n\
    \x20 %% - a single %",
    example = "\
    {command_name} -u \"postgres://usr:pwd@localhost:5432/db\" \\\n\
    \x20      -f \"host='%h' port='%p' db='%A' user='%U' pwd='%P'\"\n\
    host='localhost' port='5432' db='db' user='usr' pwd='pwd'\n\n\
    {command_name} -u \"https://www.google.com/search?q=rust+furl\" \\\n\
    \x20      -f \"scheme='%s' query='%q' path='%a'\"\n\
    scheme='https' query='q=rust+furl' path='/search'\n\n\
    {command_name} -u \"postgres://usr:pwd@localhost:5432/db\"\n\
    postgres localhost 5432 db usr pwd"
)]
struct Args {
    /// the format to use [default: "%s %h %p %A %U %P %q %f"]
    #[argh(option, short = 'f', default = "String::from(\"%s %h %p %A %U %P %q %f\")")]
    format: String,

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

    let mut ret = "".to_string();
    let mut prev_percent = false;
    for c in args.format.chars() {
        match prev_percent {
            true => {
                prev_percent = false;
                match c {
                    'a' => ret.push_str(url.path()),
                    'A' => ret.push_str({
                        let path = url.path();
                        match path.starts_with('/') {
                            true => &path[1..],
                            false => path,
                        }
                    }),
                    'f' => ret.push_str(url.fragment().unwrap_or("")),
                    'h' => ret.push_str(url.host_str().unwrap_or("")),
                    'P' => ret.push_str(url.password().unwrap_or("")),
                    'p' => ret.push_str(
                        {
                            match url.port().unwrap_or(0) {
                                0 => String::from(""),
                                v => v.to_string(),
                            }
                        }
                        .as_str(),
                    ),
                    'q' => ret.push_str(url.query().unwrap_or("")),
                    's' => ret.push_str(url.scheme()),
                    'U' => ret.push_str(url.username()),
                    'n' => ret.push_str("\n"),
                    't' => ret.push_str("\t"),
                    _ => ret.push(c),
                };
            }
            false => match c {
                '%' => prev_percent = true,
                _ => ret.push(c),
            },
        }
    }
    println!("{}", ret);
    Ok(())
}
