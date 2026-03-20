use percent_encoding::percent_decode_str;
use url::Url;

fn maybe_decode(s: &str, decode: bool) -> String {
    if decode {
        percent_decode_str(s).decode_utf8_lossy().into_owned()
    } else {
        s.to_string()
    }
}

pub fn format_url(fmt: &str, url: &Url, decode: bool) -> String {
    let mut ret = String::new();
    let mut prev_percent = false;
    for c in fmt.chars() {
        if prev_percent {
            prev_percent = false;
            match c {
                'a' => ret.push_str(&maybe_decode(url.path(), decode)),
                'A' => ret.push_str(&maybe_decode(
                    url.path().strip_prefix('/').unwrap_or(url.path()),
                    decode,
                )),
                'f' => ret.push_str(&maybe_decode(url.fragment().unwrap_or(""), decode)),
                'h' => ret.push_str(url.host_str().unwrap_or("")),
                'P' => ret.push_str(&maybe_decode(url.password().unwrap_or(""), decode)),
                'p' => ret.push_str(&url.port().map(|v| v.to_string()).unwrap_or_default()),
                'q' => ret.push_str(&maybe_decode(url.query().unwrap_or(""), decode)),
                's' => ret.push_str(url.scheme()),
                'U' => ret.push_str(&maybe_decode(url.username(), decode)),
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
