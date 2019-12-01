use vinci_core::CompileError;
use std::process::{Command, Stdio};
use std::io::{Read, Stdout, Write};

async fn compile() -> Result<usize, CompileError> {
    // find java first action
    let is_installed = is_java_installed().await;

    // check packages

    Ok(0)
}

async fn is_java_installed() -> Result<String, ()> {
    #[cfg(target_os = "macos")]
        let all_java_versions = check_and_return_out(
        Command::new("/usr/libexec/java_home")
            .arg(" -V"));

    #[cfg(target_os = "linux")]
        let all_java_versions = check_and_return_out(
        Command::new("/usr/bin/update-alternatives")
            .arg(" --config java"));

    //todo make request from config for concrete jdk version
    // here
    check_and_return_out(Command::new("/usr/bin/java")
        .arg("--version"))
}

#[cfg(target_os = "linux")]
fn check_and_return_out(out: &mut Command) -> Result<String, ()> {
    out.spawn()
        .map_err(|e| eprintln!("{}", e))
        .and_then(|child| {
            let std_in = child.stdin
                .map(|mut std_in| std_in.write_all(b"\n"));
            let out_string = child.stdout
                .map(|child_out|
                    child_out.bytes().collect::<Vec<_>>()).unwrap_or(Default::default());
            String::from_utf8(out_string.into_iter().map(|b| b.unwrap_or(0)).collect())
                .map_err(|e| eprintln!("{}", e))
        })
}

#[cfg(target_os = "macos")]
fn check_and_return_out(out: &mut Command) -> Result<String, ()> {
    out.output()
        .map_err(|e| eprintln!("{}", e))
        .and_then(|child|
            String::from_utf8(child.stdout)
                .map_err(|e| eprintln!("{}", e))
        )
}

#[cfg(test)]
mod test {
    use crate::{compile, is_java_installed};

    #[test]
    fn should_check_java_installed() {
        //haven ideas how write tests // todo about it
        async_std::task::block_on(async {
            let result = is_java_installed().await;
            println!("{:?}", result);
        });
    }
}