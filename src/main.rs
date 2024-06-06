mod cli;
mod keyboard;
mod mouse;

fn main() -> anyhow::Result<()> {
    cli::parse()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn root() {
        let mut cmd = Command::cargo_bin("ka").unwrap();
        cmd.arg("-h");
        cmd.assert().success();
        let expected_output = format!(
            r###"Usage: ka <COMMAND>

Commands:
  keyboard-and-mouse  Use keyboard and mouse to keep your machine awake [default random mouse movements & tab & windows/super/command]
  keyboard            Use keyboard to keep your machine awake [default tab & windows/super/command]
  mouse               Use mouse to keep your machine awake [default random mouse movements]
  help                Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
"###
        );
        let actual_output = String::from_utf8(cmd.assert().get_output().to_owned().stdout).unwrap();
        assert!(actual_output.contains(&expected_output));
    }

    #[test]
    fn keyboard_and_mouse() {
        let mut cmd = Command::cargo_bin("ka").unwrap();
        cmd.arg("keyboard-and-mouse");
        cmd.arg("-h");
        cmd.assert().success();
        let expected_output = format!(
            r###"Use keyboard and mouse to keep your machine awake [default random mouse movements & tab & windows/super/command]

Usage: ka keyboard-and-mouse [OPTIONS] <TIME>

Arguments:
  <TIME>  How long you want this command to run

Options:
  -t, --timeunit <TIMEUNIT>  Timeunit [default: minutes] [possible values: hours, minutes]
  -h, --help                 Print help
  -V, --version              Print version
"###
        );
        let actual_output = String::from_utf8(cmd.assert().get_output().to_owned().stdout).unwrap();
        assert!(actual_output.contains(&expected_output));
    }
}
