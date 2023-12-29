mod cli;
mod mouse;
mod settings;
mod programname;
mod keyboard;

fn main() {
    cli::parse();
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn main() {
    let mut cmd = Command::cargo_bin("ka").unwrap();
    cmd.arg("-h");
    cmd.assert().success();
    let expected_output = format!(
        r###"Keep you machine awake

Usage: ka [COMMAND]

Commands:
  mouse        Use mouse to keep your machine awake
  programname  Set name for this program
  keyboard     Use keyboard to keep your machine awake [default tab & windows/super/command]
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
"###
    );
    let actual_output = String::from_utf8(cmd.assert().get_output().to_owned().stdout).unwrap();
    assert!(actual_output.contains(&expected_output));
    }

}