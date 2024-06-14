use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn run(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "run", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn cache(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "cache", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn info(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "info", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn verify(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "verify", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn help(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "help", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn version(args: String) -> FnResult<String> {
    dag().call(
        "https://pkg.fluentci.io/bun@v0.7.1?wasm=1",
        "setup",
        vec!["latest"],
    )?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "bunx", "cypress", "version", &args])?
        .stdout()?;
    Ok(stdout)
}
