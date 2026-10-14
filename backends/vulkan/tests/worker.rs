use kuiper_contracts::{MAX_MESSAGE, Response, canonical};
use std::io::Write;
use std::process::{Command, Stdio};
fn transact(bytes: &[u8]) -> Vec<Response> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuiper-vulkan-worker"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        child.stdin.take().unwrap().write_all(bytes).unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
        .stdout
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| canonical::parse(line).unwrap())
        .collect()
}
#[test]
fn describe_declares_both_runtime_output_abi_and_execution_schema() {
    let result = transact(b"{\"method\":\"describe\"}\n");
    let Response::Description { id, endpoints, .. } = &result[0] else {
        panic!("missing description")
    };
    assert_eq!(id, kuiper_vulkan::RUNTIME_ID);
    assert!(
        endpoints[0]
            .produces
            .contains(&kuiper_contracts::WORD_ABI.to_owned())
    );
    assert!(
        endpoints[0]
            .produces
            .contains(&"kuiper.execution/1".to_owned())
    );
}
#[test]
fn unknown_and_duplicate_required_fields_are_rejected() {
    let result=transact(b"{\"method\":\"describe\",\"required_fact\":true}\n{\"method\":\"describe\",\"method\":\"describe\"}\n");
    assert_eq!(result.len(), 2);
    assert!(
        result
            .iter()
            .all(|r| matches!(r, Response::Rejected { .. }))
    );
}
#[test]
fn oversized_requests_are_rejected_before_parsing() {
    let result = transact(&vec![b'x'; MAX_MESSAGE + 1]);
    let Response::Rejected { diagnostic } = &result[0] else {
        panic!("oversize request admitted")
    };
    assert_eq!(diagnostic.code, "message-too-large");
}
