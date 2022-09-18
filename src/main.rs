use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Formatter for URLs using a sprintf-like template.
///
/// $ furl -u "postgres://usr:pwd@localhost:5432/db" -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'"
///
/// host='localhost' port='5432' db='db' user='usr' pwd='pwd'
///
/// $ furl -u "https://www.google.com/search?q=rust+furl" -f "scheme='%s' query='%q' path='%a'"
///
/// scheme='https' query='q=rust+furl' path='/search'
///
/// $ furl -u "https://en.wikipedia.org/wiki/Rust#Prevention" -f "port='%p' fragment='%f'"
///
/// port='' fragment='Prevention'
///
/// $ furl -u "postgres://usr:pwd@localhost:5432/db"
///
/// postgres localhost 5432 db usr pwd  
///
/// For the last example, the default format is used (see below).
struct Args {
    /// The format to use.{n}
    ///    %A - the path, without the starting '/'{n}
    ///    %a - the path{n}
    ///    %f - the fragment{n}
    ///    %h - the hostname{n}
    ///    %P - the password of the userinfo portion{n}
    ///    %p - the port{n}
    ///    %q - the query string{n}
    ///    %s - the scheme{n}
    ///    %U - the username of the userinfo portion{n}
    ///    %n - newline (\t)
    ///    %t - tab (\n)
    ///    %% - a single %
    #[clap(short, long, value_parser, default_value = "%s %h %p %A %U %P %q %f")]
    format: String,

    /// The URL to parse and format
    #[clap(short, long, value_parser)]
    url: String,
}

#[derive(Debug)]
enum AppErr {
    UrlParseError(url::ParseError),
}

impl From<url::ParseError> for AppErr {
    fn from(err: url::ParseError) -> Self {
        AppErr::UrlParseError(err)
    }
}

fn main() -> Result<(), AppErr> {
    let args = Args::parse();
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
