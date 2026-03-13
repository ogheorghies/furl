use std::process::Command;

#[derive(PartialEq, Debug)]
struct Res {
    out: String,
    err: String,
    status: i32,
}

impl Res {
    pub fn new(out: &str, err: &str, status: i32) -> Self {
        Res {
            out: out.to_string(),
            err: err.to_string(),
            status,
        }
    }
}

fn success(out: &str) -> Res {
    Res::new(out, "", 0)
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
fn default_format() {
    assert_eq!(
        run(&["-u", "postgres://usr:pwd@localhost:5432/db"]),
        success("postgres localhost 5432 db usr pwd  \n")
    );
}

#[test]
fn handles_wrong_args() {
    let res = run(&["asdda"]);
    assert!(res.err.contains("Unrecognized argument: asdda"));

    assert!(res.status != 0);
}

#[test]
fn handles_wrong_url() {
    let res = run(&["-u", "hrt/asd/asd/"]);
    assert!(res.err.contains("invalid URL"));

    assert!(res.status != 0);
}
