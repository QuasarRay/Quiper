use kuiper_contracts::{Diagnostic, MAX_MESSAGE, PROTOCOL, Request, Response, canonical};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
fn handle(request: Request) -> Response {
    match request {
        Request::Describe {} => Response::Description {
            protocol: PROTOCOL.into(),
            id: kuiper_vulkan::RUNTIME_ID.into(),
            endpoints: kuiper_vulkan::endpoints(),
        },
        Request::Execute {
            artifact,
            invocation,
        } => match kuiper_vulkan::execute(&artifact, &invocation) {
            Ok(execution) => Response::Executed { execution },
            Err(diagnostic) => Response::Rejected { diagnostic },
        },
        Request::Compile { .. } => Response::Rejected {
            diagnostic: Diagnostic::new(
                "vulkan",
                "unsupported-method",
                "runtime worker does not compile",
            ),
        },
    }
}
fn run() -> io::Result<()> {
    let mut input = BufReader::new(io::stdin().lock());
    let mut output = BufWriter::new(io::stdout().lock());
    loop {
        let mut line = vec![];
        let count = input
            .by_ref()
            .take(MAX_MESSAGE as u64 + 1)
            .read_until(b'\n', &mut line)?;
        if count == 0 {
            break;
        }
        let too_large = line.len() > MAX_MESSAGE;
        let response = if too_large {
            Response::Rejected {
                diagnostic: Diagnostic::new(
                    "protocol",
                    "message-too-large",
                    "request exceeds the message limit",
                ),
            }
        } else {
            match canonical::parse::<Request>(&line) {
                Ok(request) => handle(request),
                Err(diagnostic) => Response::Rejected { diagnostic },
            }
        };
        output.write_all(&canonical::encode(&response).map_err(io::Error::other)?)?;
        output.write_all(b"\n")?;
        output.flush()?;
        if too_large {
            break;
        }
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
