use url::Url;

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < '\x20' => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

pub fn json_url(url: &Url, default_port: bool) -> String {
    let mut fields: Vec<String> = Vec::new();

    // Fields in URL order: scheme://user:password@host:port/path?query#fragment
    fields.push(format!("\"scheme\":\"{}\"", escape(url.scheme())));

    if !url.username().is_empty() {
        fields.push(format!("\"user\":\"{}\"", escape(url.username())));
    }

    if let Some(password) = url.password() {
        fields.push(format!("\"password\":\"{}\"", escape(password)));
    }

    if let Some(host) = url.host_str() {
        fields.push(format!("\"host\":\"{}\"", escape(host)));
    }

    let port = if default_port {
        url.port_or_known_default()
    } else {
        url.port()
    };
    if let Some(port) = port {
        fields.push(format!("\"port\":{port}"));
    }

    let path = url.path();
    if path != "/" && !path.is_empty() {
        fields.push(format!("\"path\":\"{}\"", escape(path)));
    }

    if let Some(query) = url.query() {
        let mut pairs: Vec<String> = Vec::new();
        for part in query.split('&') {
            if let Some((k, v)) = part.split_once('=') {
                pairs.push(format!("\"{}\":\"{}\"", escape(k), escape(v)));
            } else {
                pairs.push(format!("\"{}\":\"\"", escape(part)));
            }
        }
        fields.push(format!("\"query\":{{{}}}", pairs.join(",")));
    }

    if let Some(fragment) = url.fragment() {
        fields.push(format!("\"fragment\":\"{}\"", escape(fragment)));
    }

    format!("{{{}}}", fields.join(","))
}
