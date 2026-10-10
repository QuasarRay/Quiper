use kuiper_contracts::{Diagnostic, MAX_MESSAGE, PROTOCOL, Request, Response, canonical};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

fn handle(request: Request) -> Response {
    match request {
        Request::Describe {} => Response::Description {
            protocol: PROTOCOL.into(),
            id: kuiper_spirt::COMPILER_ID.into(),
            endpoints: kuiper_spirt::endpoints(),
        },
        Request::Compile { package, entry } => match kuiper_spirt::compile(&package, &entry) {
            Ok(artifact) => Response::Compiled { artifact },
            Err(diagnostic) => Response::Rejected { diagnostic },
        },
        Request::Execute { .. } => Response::Rejected {
            diagnostic: Diagnostic::new(
                "spirt",
                "unsupported-method",
                "compiler worker does not execute",
            ),
        },
    }
}
fn run() -> io::Result<()> {
    let mut input = BufReader::new(io::stdin().lock());
    let mut output = BufWriter::new(io::stdout().lock());
    loop {
        let mut line = Vec::new();
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
        let bytes = canonical::encode(&response).map_err(io::Error::other)?;
        output.write_all(&bytes)?;
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
