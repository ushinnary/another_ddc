
#[macro_export]
macro_rules! run_ddcutil_command {
    ($($arg:expr),*) => {
        {
            let mut command = std::process::Command::new("ddcutil");
            $(
                command.arg($arg);
            )*
             let output = command.output().expect("failed to execute process");

            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();

            if !stderr.is_empty() && stdout.is_empty() {
                panic!("ddcutil error: {}", stderr);
            }

            stdout
        }
    };
}
