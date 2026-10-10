use kuiper_contracts::{MAX_MESSAGE, Request, Response, canonical};
use std::io::Write;
use std::process::{Command, Stdio};

fn transact(bytes: &[u8]) -> Vec<Response> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuiper-spirt-worker"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(bytes).unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| canonical::parse(line).unwrap())
        .collect()
}
#[test]
fn describe_and_invalid_required_fields_are_checked() {
    let bytes = concat!(
        "{\"method\":\"describe\"}\n",
        "{\"method\":\"describe\",\"method\":\"describe\"}\n",
        "{\"method\":\"describe\",\"required_fact\":true}\n",
        "{\"method\":\"compile\",\"entry\":\"x\",\"package\":{\"schema\":\"kuiper.kir/1\",\"profile\":\"kuiper.integer32/1\",\"kernels\":[{\"name\":\"x\",\"local_size\":[4,1,1],\"resources\":[],\"parameters\":[],\"body\":{\"instructions\":[{\"kind\":\"barrier\"}],\"outputs\":[]}}]}}\n"
    );
    let responses = transact(bytes.as_bytes());
    assert_eq!(responses.len(), 4);
    let Response::Description { id, .. } = &responses[0] else {
        panic!("missing description")
    };
    assert_eq!(id, kuiper_spirt::COMPILER_ID);
    assert!(
        responses[1..]
            .iter()
            .all(|r| matches!(r, Response::Rejected { .. }))
    );
}
#[test]
fn compile_returns_a_bound_artifact() {
    let package = canonical::parse(include_bytes!("fixtures/pretested-loop.json")).unwrap();
    let digest = canonical::digest(&package).unwrap();
    let mut request = canonical::encode(&Request::Compile {
        package,
        entry: "pretested_loop".into(),
    })
    .unwrap();
    request.push(b'\n');
    let responses = transact(&request);
    assert_eq!(responses.len(), 1);
    let Response::Compiled { artifact } = &responses[0] else {
        panic!("{:#?}", responses[0])
    };
    assert_eq!(artifact.parent_digest, digest);
}
#[test]
fn message_limit_is_enforced_before_parsing() {
    let responses = transact(&vec![b'x'; MAX_MESSAGE + 1]);
    assert_eq!(responses.len(), 1);
    let Response::Rejected { diagnostic } = &responses[0] else {
        panic!("oversized request accepted")
    };
    assert_eq!(diagnostic.code, "message-too-large");
}
