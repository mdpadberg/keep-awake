# Keep-awake (ka)
Keep-awake is a cli tool that can be used to keep your machine awake.

## Install on Linux
```
brew install mdpadberg/tap/ka
```

## Install on Windows
```
scoop bucket add mdpadberg https://github.com/mdpadberg/scoop-bucket.git
scoop install ka
```

> :warning: Maybe you need to run your terminal in admin mode to make keep awake work

## Install on macOS
```
brew install mdpadberg/tap/ka
```
> Note:
Keep awake is a fully open-sourced project and will never collect any user data! However if you want to use the keyboard function, you need to enable Accessibility permissions. Add your default terminal in the allowed Accessibility apps:
![screenshot of Accessibility menus in macos](macos-security-and-privacy.png)

## Examples
```console
% ka -h
Usage: ka <COMMAND>

Commands:
  keyboard-and-mouse  Use keyboard and mouse to keep your machine awake [default random mouse movements & tab & windows/super/command]
  keyboard            Use keyboard to keep your machine awake [default tab & windows/super/command]
  mouse               Use mouse to keep your machine awake [default random mouse movements]
  help                Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

### Subcommand: Mouse
Use mouse to keep your machine awake

```console
% ka mouse -h
Use mouse to keep your machine awake [default random mouse movements]

Usage: ka mouse [OPTIONS] <TIME>

Arguments:
  <TIME>  How long you want this command to run

Options:
  -t, --timeunit <TIMEUNIT>  Timeunit [default: minutes] [possible values: hours, minutes]
  -h, --help                 Print help
  -V, --version              Print version
```

### Subcommand: Keyboard
Use keyboard to keep your machine awake [default tab & windows/super/command]

```console
% ka keyboard -h
Use keyboard to keep your machine awake [default tab & windows/super/command]

Usage: ka keyboard [OPTIONS] <TIME>

Arguments:
  <TIME>  How long you want this command to run

Options:
  -t, --timeunit <TIMEUNIT>  Timeunit [default: minutes] [possible values: hours, minutes]
  -h, --help                 Print help
  -V, --version              Print version
```

### Subcommand: Keyboard-and-mouse
Set name for this program

```console
% ka keyboard-and-mouse -h
Use keyboard and mouse to keep your machine awake [default random mouse movements & tab & windows/super/command]

Usage: ka keyboard-and-mouse [OPTIONS] <TIME>

Arguments:
  <TIME>  How long you want this command to run

Options:
  -t, --timeunit <TIMEUNIT>  Timeunit [default: minutes] [possible values: hours, minutes]
  -h, --help                 Print help
  -V, --version              Print version
```
