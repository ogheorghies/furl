use std::fmt;
use url::Url;

pub struct UrlParts<'a> {
    pub scheme: &'a str,
    pub user: Option<&'a str>,
    pub password: Option<&'a str>,
    pub host: Option<&'a str>,
    pub port: Option<u16>,
    pub path: Option<&'a str>,
    pub query: Option<Vec<(&'a str, &'a str)>>,
    pub fragment: Option<&'a str>,
}

impl<'a> UrlParts<'a> {
    pub fn from_url(url: &'a Url, default_port: bool) -> Self {
        let user = if url.username().is_empty() {
            None
        } else {
            Some(url.username())
        };

        let port = if default_port {
            url.port_or_known_default()
        } else {
            url.port()
        };

        let path = match url.path() {
            "/" | "" => None,
            p => Some(p),
        };

        let query = url.query().map(|q| {
            q.split('&')
                .map(|part| {
                    part.split_once('=').unwrap_or((part, ""))
                })
                .collect()
        });

        UrlParts {
            scheme: url.scheme(),
            user,
            password: url.password(),
            host: url.host_str(),
            port,
            path,
            query,
            fragment: url.fragment(),
        }
    }

}

impl fmt::Display for UrlParts<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = false;

        macro_rules! comma {
            ($f:expr) => {
                if sep { write!($f, ",")?; }
                #[allow(unused_assignments)]
                { sep = true; }
            };
        }

        write!(f, "{{")?;

        comma!(f);
        write!(f, "\"scheme\":\"")?;
        write_escaped(f, self.scheme)?;
        write!(f, "\"")?;

        if let Some(user) = self.user {
            comma!(f);
            write!(f, "\"user\":\"")?;
            write_escaped(f, user)?;
            write!(f, "\"")?;
        }

        if let Some(password) = self.password {
            comma!(f);
            write!(f, "\"password\":\"")?;
            write_escaped(f, password)?;
            write!(f, "\"")?;
        }

        if let Some(host) = self.host {
            comma!(f);
            write!(f, "\"host\":\"")?;
            write_escaped(f, host)?;
            write!(f, "\"")?;
        }

        if let Some(port) = self.port {
            comma!(f);
            write!(f, "\"port\":{port}")?;
        }

        if let Some(path) = self.path {
            comma!(f);
            write!(f, "\"path\":\"")?;
            write_escaped(f, path)?;
            write!(f, "\"")?;
        }

        if let Some(query) = &self.query {
            comma!(f);
            write!(f, "\"query\":{{")?;
            let mut qsep = false;
            for (k, v) in query {
                if qsep { write!(f, ",")?; }
                qsep = true;
                write!(f, "\"")?;
                write_escaped(f, k)?;
                write!(f, "\":\"")?;
                write_escaped(f, v)?;
                write!(f, "\"")?;
            }
            write!(f, "}}")?;
        }

        if let Some(fragment) = self.fragment {
            comma!(f);
            write!(f, "\"fragment\":\"")?;
            write_escaped(f, fragment)?;
            write!(f, "\"")?;
        }

        write!(f, "}}")
    }
}

fn write_escaped(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    for c in s.chars() {
        match c {
            '"' => f.write_str("\\\"")?,
            '\\' => f.write_str("\\\\")?,
            '\n' => f.write_str("\\n")?,
            '\r' => f.write_str("\\r")?,
            '\t' => f.write_str("\\t")?,
            c if c < '\x20' => write!(f, "\\u{:04x}", c as u32)?,
            c => write!(f, "{c}")?,
        }
    }
    Ok(())
}
