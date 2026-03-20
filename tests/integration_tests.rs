use std::process::Command;

#[derive(PartialEq, Debug)]
struct Res {
    out: String,
    err: String,
    status: i32,
}

fn success(out: &str) -> Res {
    Res {
        out: out.to_string(),
        err: String::new(),
        status: 0,
    }
}

fn run(args: &[&str]) -> Res {
    let bin = env!("CARGO_BIN_EXE_furl");

    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to execute process");

    Res {
        out: String::from_utf8_lossy(&output.stdout).to_string(),
        err: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status.code().unwrap_or(-1),
    }
}

#[test]
fn help() {
    let res = run(&["--help"]);
    assert!(res.out.contains("furl"));
    assert!(res.out.contains("postgres"));
    assert!(res.out.contains("Format specifiers"));
}

#[test]
fn path() {
    assert_eq!(run(&["-u", "http://example.com/", "-f", "%a"]), success("/\n"));
    assert_eq!(run(&["-u", "http://example.com/", "-f", "%A"]), success("\n"));
}

#[test]
fn port() {
    assert_eq!(run(&["-u", "http://example.com/", "-f", "%p"]), success("\n"));
    assert_eq!(
        run(&["-u", "http://example.com:8080/", "-f", "%p"]),
        success("8080\n")
    );
}

#[test]
fn multiple_fields() {
    assert_eq!(
        run(&["-u", "http://example.com:8080/a#b", "-f", "%s %h %p %a %f"]),
        success("http example.com 8080 /a b\n")
    );
}

#[test]
fn postgres_url() {
    assert_eq!(
        run(&["-u", "postgres://usr:pwd@localhost:5432/db", "-f", "host='%h' port='%p' db='%A' user='%U' pwd='%P'"]),
        success("host='localhost' port='5432' db='db' user='usr' pwd='pwd'\n")
    );
    assert_eq!(
        run(&["-u", "postgres://usr@localhost:5432/db", "-f", "host='%h' port='%p' db='%A' user='%U' pwd='%P'"]),
        success("host='localhost' port='5432' db='db' user='usr' pwd=''\n")
    );
}

#[test]
fn username() {
    assert_eq!(
        run(&["-u", "http://alice@example.com/", "-f", "%U"]),
        success("alice\n")
    );
    assert_eq!(
        run(&["-u", "http://example.com/", "-f", "%U"]),
        success("\n")
    );
}

#[test]
fn query() {
    assert_eq!(
        run(&["-u", "https://www.google.com/search?q=rust+furl", "-f", "scheme='%s' query='%q' path='%a'"]),
        success("scheme='https' query='q=rust+furl' path='/search'\n")
    );
}

#[test]
fn escape_sequences() {
    assert_eq!(
        run(&["-u", "http://example.com/", "-f", "%a%t%%%n[%p]"]),
        success("/\t%\n[]\n")
    );
}

#[test]
fn trailing_percent() {
    assert_eq!(
        run(&["-u", "http://example.com/", "-f", "hello%"]),
        success("hello%\n")
    );
}

#[test]
fn unknown_specifier() {
    assert_eq!(
        run(&["-u", "http://example.com/", "-f", "%z"]),
        success("%z\n")
    );
}

#[test]
fn json_full() {
    assert_eq!(
        run(&["-u", "postgres://usr:pwd@localhost:5432/db", "-j"]),
        success("{\"scheme\":\"postgres\",\"user\":\"usr\",\"password\":\"pwd\",\"host\":\"localhost\",\"port\":5432,\"path\":\"/db\"}\n")
    );
}

#[test]
fn json_with_query() {
    assert_eq!(
        run(&["-u", "https://www.google.com/search?q=rust+furl&lang=en", "-j"]),
        success("{\"scheme\":\"https\",\"host\":\"www.google.com\",\"path\":\"/search\",\"query\":{\"q\":\"rust+furl\",\"lang\":\"en\"}}\n")
    );
}

#[test]
fn json_minimal() {
    assert_eq!(
        run(&["-u", "http://example.com/", "-j"]),
        success("{\"scheme\":\"http\",\"host\":\"example.com\"}\n")
    );
}

#[test]
fn json_with_fragment() {
    assert_eq!(
        run(&["-u", "http://example.com/page#section", "-j"]),
        success("{\"scheme\":\"http\",\"host\":\"example.com\",\"path\":\"/page\",\"fragment\":\"section\"}\n")
    );
}

#[test]
fn default_format() {
    assert_eq!(
        run(&["-u", "postgres://usr:pwd@localhost:5432/db"]),
        success("postgres localhost 5432 db usr pwd  \n")
    );
}

#[test]
fn handles_wrong_args() {
    let res = run(&["--bogus"]);
    assert!(res.err.contains("Unrecognized argument: --bogus"));
    assert!(res.status != 0);
}

#[test]
fn missing_url() {
    let res = run(&["-j"]);
    assert!(res.err.contains("missing URL"));
    assert!(res.status != 0);
}

#[test]
fn positional_url() {
    assert_eq!(
        run(&["http://example.com/", "-f", "%h"]),
        success("example.com\n")
    );
}

#[test]
fn positional_url_json() {
    assert_eq!(
        run(&["-j", "http://example.com/"]),
        success("{\"scheme\":\"http\",\"host\":\"example.com\"}\n")
    );
}

#[test]
fn handles_wrong_url() {
    let res = run(&["-u", "hrt/asd/asd/"]);
    assert!(res.err.contains("invalid URL"));

    assert!(res.status != 0);
}

#[test]
fn decode_path() {
    assert_eq!(
        run(&["-u", "https://example.com/caf%C3%A9/m%C3%BCsli", "-f", "%a"]),
        success("/café/müsli\n")
    );
}

#[test]
fn decode_path_encoded_flag() {
    assert_eq!(
        run(&["-u", "https://example.com/caf%C3%A9", "-f", "%a", "-e"]),
        success("/caf%C3%A9\n")
    );
}

#[test]
fn decode_query() {
    assert_eq!(
        run(&["-u", "https://example.com/?q=hello%20world", "-f", "%q"]),
        success("q=hello world\n")
    );
}

#[test]
fn decode_fragment() {
    assert_eq!(
        run(&["-u", "https://example.com/#caf%C3%A9", "-f", "%f"]),
        success("café\n")
    );
}

#[test]
fn decode_user_password() {
    assert_eq!(
        run(&["-u", "https://us%40er:p%40ss@example.com/", "-f", "%U %P"]),
        success("us@er p@ss\n")
    );
}

#[test]
fn decode_json() {
    assert_eq!(
        run(&["-u", "https://example.com/caf%C3%A9?q=hello%20world#s%C3%A9c", "-j"]),
        success("{\"scheme\":\"https\",\"host\":\"example.com\",\"path\":\"/café\",\"query\":{\"q\":\"hello world\"},\"fragment\":\"séc\"}\n")
    );
}

#[test]
fn decode_json_encoded_flag() {
    assert_eq!(
        run(&["-u", "https://example.com/caf%C3%A9", "-j", "-e"]),
        success("{\"scheme\":\"https\",\"host\":\"example.com\",\"path\":\"/caf%C3%A9\"}\n")
    );
}
