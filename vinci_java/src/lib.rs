use vinci_core::{CompileError, StreamExt, async_await::FusedFuture, async_walk_and_find_all_files};
use std::process::{Command, Stdio};
use std::io::{Read, Stdout, Write};

// GLOBAL TODO make in with trait and in diff mods, and all same things should be in the CORE!!!!

const USER_BIN: &str = "/usr/bin/";
const JAVA_HOME: &str = "JAVA_HOME";
const JAVA_PACKAGES: [&'static str; 6] = [
    "javac",
    "jar",
    "java",
    "jarsigner",
    "javadoc",
    "javah", ];

///*not so important (exists since jdk 1.6) */"/usr/bin/javapackager"

async fn compile(r#in: String) -> Result<usize, CompileError> {
    // find java first action
    let is_installed = get_java_installed_version().await;

    if is_installed.is_err() {
        return Err(CompileError::new(
            "java".into(),
            "yours files".into(),
            "Java missing, want to install it? [Y/n]".into()));
    }
    // set java home
    // find by which java!!
    let java_path = if let Ok(java_home) = check_java_home().await {
        java_home
    } else {
        USER_BIN.to_owned()
    };
    // check packages
    let all_packages_exists = is_all_java_packages_exists(&java_path).await;
    // find all files in target dir
    let java_files = async_walk_and_find_all_files(&r#in).await;

    Ok(0)
}

async fn check_java_home() -> Result<String, ()> {
    std::env::var(JAVA_HOME)
        .map_err(|e| eprintln!("{}", e))
        .and_then(|home| if home.is_empty() { Err(()) } else { Ok(home) })
}

async fn get_java_installed_version() -> Result<String, ()> {

    //todo make request from config for concrete jdk version
    // here
//    #[cfg(target_os = "macos")]
//        let all_java_versions = check_and_return_out(
//        Command::new("/usr/libexec/java_home")
//            .arg(" -V"));
//
//    #[cfg(target_os = "linux")]
//        let all_java_versions = check_and_return_out(
//        Command::new("/usr/bin/update-alternatives")
//            .arg(" --config java"));


    check_and_return_err_out(Command::new("java")
        .arg("-version"))
        .map(|java| {
            println!("Found {}", java);
            Ok(java)
        })
        .unwrap_or(Err(()))
}

fn check_and_return_err_out(out: &mut Command) -> Result<String, ()> {
    out.output()
        .map_err(|e| eprintln!("{}", e))
        .and_then(|child|
            String::from_utf8(child.stderr)
                .map_err(|e| eprintln!("{}", e))
        )
}

async fn is_all_java_packages_exists(java_path: &str) -> bool {
    let files = JAVA_PACKAGES.iter()
        .map(|java_bin| format!("{}{}", java_path, java_bin))
        .collect::<Vec<_>>();
    let mut exists = vec![];
    for file in files.iter() {
        let package_path = std::path::Path::new(file).exists();
        exists.push(package_path);
    }
    exists.into_iter().all(|is| is)
}

#[cfg(test)]
mod test {
    use crate::{compile, get_java_installed_version, is_all_java_packages_exists, JAVA_HOME, USER_BIN, async_walk_and_find_all_files};

    #[test]
    fn should_check_java_installed() {
        //haven ideas how write tests // todo about it
        vinci_core::block_on(async {
            let result = get_java_installed_version().await;
            println!("{:?}", result);
        });
    }

    #[test]
    fn should_check_all_java_packages_is_installed() {
        //haven ideas how write tests // todo about it
        vinci_core::block_on(async {
            let result = is_all_java_packages_exists(USER_BIN).await;
            println!("{:?}", result);
        });
    }
}
