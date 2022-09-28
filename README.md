# Keep-awake
Keep-awake is a cli tool that can be used to keep your machine awake.

## Install

### macOS
```
brew install mdpadberg/tap/keep-awake
```

### Linux
```
brew install mdpadberg/tap/keep-awake
```

### Windows   
Go to the release page and download the latest release: https://github.com/mdpadberg/keep-awake/releases

## Examples
```console
% keep-awake -h
keep-awake 0.1.2

USAGE:
    keep-awake [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help     Print this message or the help of the given subcommand(s)
    mouse    Use mouse to keep your machine awake

```

### Subcommand: Mouse
Use mouse to keep your machine awake

```console
% keep-awake mouse -h
USAGE:
    keep-awake mouse [OPTIONS]

OPTIONS:
    -h, --help                   Print help information
        --height <height>        set your monitors height in pixels [default: 1080]
        --interval <interval>    set a time interval in seconds on how often you want to run this
                                 [default: 60]
        --width <width>          set your monitors width in pixels [default: 1920]
```