use url::Url;

pub fn format_url(fmt: &str, url: &Url) -> String {
    let mut ret = String::new();
    let mut prev_percent = false;
    for c in fmt.chars() {
        if prev_percent {
            prev_percent = false;
            match c {
                'a' => ret.push_str(url.path()),
                'A' => ret.push_str(url.path().strip_prefix('/').unwrap_or(url.path())),
                'f' => ret.push_str(url.fragment().unwrap_or("")),
                'h' => ret.push_str(url.host_str().unwrap_or("")),
                'P' => ret.push_str(url.password().unwrap_or("")),
                'p' => ret.push_str(&url.port().map(|v| v.to_string()).unwrap_or_default()),
                'q' => ret.push_str(url.query().unwrap_or("")),
                's' => ret.push_str(url.scheme()),
                'U' => ret.push_str(url.username()),
                'n' => ret.push('\n'),
                't' => ret.push('\t'),
                '%' => ret.push('%'),
                _ => {
                    ret.push('%');
                    ret.push(c);
                }
            };
        } else if c == '%' {
            prev_percent = true;
        } else {
            ret.push(c);
        }
    }
    if prev_percent {
        ret.push('%');
    }
    ret
}
