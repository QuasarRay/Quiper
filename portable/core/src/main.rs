use kuiper_contracts::{canonical, validate, *};
use kuiper_core::{host, reference, worker};
use std::path::Path;

fn read<T: serde::de::DeserializeOwned>(path: &str) -> Result<T> {
    let bytes = std::fs::read(path).map_err(|e| Diagnostic::new("cli", "io", e.to_string()))?;
    canonical::parse(&bytes)
}
fn missing() -> Diagnostic {
    Diagnostic::new(
        "cli",
        "arguments",
        "usage: kuiper-core check PACKAGE | inspect ROOT | reference PACKAGE INVOCATION | compile PACKAGE ENTRY ROOT CACHE | run PACKAGE INVOCATION ROOT --allow-experimental | reference-plan PACKAGE PLAN | run-plan PACKAGE PLAN ROOT --allow-experimental",
    )
}
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let result: Result<Vec<u8>> = (|| match arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["check", package] => {
            validate::package(&read::<Package>(package)?)?;
            canonical::encode(&serde_json::json!({"checked":true}))
        }
        ["inspect", root] => canonical::encode(
            &worker::discover(Path::new(root))?
                .iter()
                .map(|w| &w.manifest)
                .collect::<Vec<_>>(),
        ),
        ["reference", package, invocation] => {
            canonical::encode(&reference::execute(&read(package)?, &read(invocation)?)?)
        }
        ["compile", package, entry, root, cache] => {
            let package: Package = read(package)?;
            let route = worker::select(&worker::discover(Path::new(root))?, &package.profile)?;
            let artifact = route.compile(&package, entry)?;
            worker::cache(Path::new(cache), &artifact)?;
            canonical::encode(&artifact)
        }
        ["run", package, invocation, root, "--allow-experimental"] => {
            let package: Package = read(package)?;
            let invocation: Invocation = read(invocation)?;
            let route = worker::select(&worker::discover(Path::new(root))?, &package.profile)?;
            let artifact = route.compile(&package, &invocation.entry)?;
            canonical::encode(&route.execute(&artifact, &invocation)?)
        }
        ["reference-plan", package, plan] => {
            canonical::encode(&host::reference_plan(&read(package)?, &read(plan)?)?)
        }
        ["run-plan", package, plan, root, "--allow-experimental"] => {
            let package: Package = read(package)?;
            let route = worker::select(&worker::discover(Path::new(root))?, &package.profile)?;
            canonical::encode(&host::run_plan(&package, &read(plan)?, &route)?)
        }
        _ => Err(missing()),
    })();
    match result {
        Ok(bytes) => {
            println!("{}", String::from_utf8_lossy(&bytes));
        }
        Err(diagnostic) => {
            eprintln!(
                "{}",
                serde_json::to_string(&diagnostic).unwrap_or_else(|_| diagnostic.to_string())
            );
            std::process::exit(1);
        }
    }
}
