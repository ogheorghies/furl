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

fn run(args: &str) -> Res {
    let cmd = test_bin::get_test_bin("furl");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", cmd.get_program().to_str().unwrap(), args))
        .output()
        .expect("failed to execute process");

    Res {
        out: String::from_utf8_lossy(&output.stdout).to_string(),
        err: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status.code().unwrap_or(-1),
    }
}

#[test]
fn works_well() {
    assert!(run("-h").out.contains("furl "));
    assert!(!run("-h").out.contains("postgres"));
    assert!(run("--help").out.contains("postgres"));
    assert_eq!(run("-u http://example.com/ -f '%a'"), success("/\n"));
    assert_eq!(run("-u http://example.com/ -f '%A'"), success("\n"));
    assert_eq!(run("-u http://example.com/ -f '%p'"), success("\n"));
    assert_eq!(
        run("-u http://example.com:8080/ -f '%p'"),
        success("8080\n")
    );
    assert_eq!(
        run("-u http://example.com:8080/a#b -f '%s %h %p %a %f'"),
        success("http example.com 8080 /a b\n")
    );
    assert_eq!(
        run(
            r#"-u "postgres://usr:pwd@localhost:5432/db" -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'""#
        ),
        success("host='localhost' port='5432' db='db' user='usr' pwd='pwd'\n")
    );
    assert_eq!(
        run(
            r#"-u "postgres://usr@localhost:5432/db" -f "host='%h' port='%p' db='%A' user='%U' pwd='%P'""#
        ),
        success("host='localhost' port='5432' db='db' user='usr' pwd=''\n")
    );
    assert_eq!(
        run(
            r#"-u "https://www.google.com/search?q=rust+furl" -f "scheme='%s' query='%q' path='%a'""#
        ),
        success("scheme='https' query='q=rust+furl' path='/search'\n")
    );
    assert_eq!(
        run("-u http://example.com/ -f '%a%t%%%n[%p]'"),
        success("/\t%\n[]\n")
    );
}

#[test]
fn works_with_default_format() {
    assert_eq!(
        run(r#"-u "postgres://usr:pwd@localhost:5432/db""#),
        success("postgres localhost 5432 db usr pwd  \n")
    );
}

#[test]
fn handles_wrong_args() {
    let res = run(r#"asdda"#);
    assert!(res
        .err
        .contains("error: Found argument 'asdda' which wasn't expected"));

    assert!(res.status != 0);
}

#[test]
fn handles_wrong_url() {
    let res = run(r#"-u hrt/asd/asd/"#);
    assert!(res.err.contains("UrlParseError"));

    assert!(res.status != 0);
}
