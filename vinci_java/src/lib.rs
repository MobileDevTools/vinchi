use vinci_core::CompileError;
use std::process::{Command, Stdio};
use std::io::{Read, Stdout, Write};
use std::borrow::{Cow, Borrow};
use std::ffi::OsStr;

// GLOBAL TODO make in with trait and in diff mods, and all same things should be in the CORE!!!!

const JAVA_PACKAGES: [&'static str; 7] = [
    "/usr/bin/javac",
    "/usr/bin/jar",
    "/usr/bin/java",
    "/usr/bin/jarsigner",
    "/usr/bin/javadoc",
    "/usr/bin/javah",
    /*not so important (exists since jdk 1.6) */"/usr/bin/javapackager"]
;

async fn compile() -> Result<usize, CompileError> {
    // find java first action
    let is_installed = is_java_installed().await;

    if is_installed.is_err() {
        return Err(CompileError::new(
            "java".into(),
            "yours files".into(),
            "Java missing, want to install it? [Y/n]".into()));
    }
    // check packages

    let all_packages_exists = is_all_java_packages_exists();

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

async fn is_all_java_packages_exists() -> bool {
    let files = JAVA_PACKAGES.iter()
        .map(|file| {
            async_std::path::Path::new(file).exists()
        })
        .collect::<Vec<_>>();
    vinci_core::future::join_all(files).await.iter().all(|is| *is)
}

#[cfg(test)]
mod test {
    use crate::{compile, is_java_installed, is_all_java_packages_exists};

    #[test]
    fn should_check_java_installed() {
        //haven ideas how write tests // todo about it
        async_std::task::block_on(async {
            let result = is_java_installed().await;
            println!("{:?}", result);
        });
    }

    #[test]
    fn should_check_all_java_packages_is_installed() {
        //haven ideas how write tests // todo about it
        async_std::task::block_on(async {
            let result = is_all_java_packages_exists().await;
            println!("{:?}", result);
        });
    }
}