# `steve-lint`

A simple command line utility to ensure all your file names are made up of only alphanumeric characters.

## Usage
```
./steve-lint [flags] [file]
```

| Argument                     | Description                                  |
| ---------------------------- | -------------------------------------------- |
| `<file>`                     | The directory/file to check                  |
| `-f`, `--fix`                | Automatically rename directories/files       |
| `-h`, `--help`               | Prints help information                      |
| `-v`, `--verbose`            | Log all directories/files traversed          |
| `-V`, `--version`            | Prints version information                   |
| `-i`, `--ignore <ignore>...` |  Glob pattern of directories/files to ignore |

## Examples

```
./steve-lint --ignore "**/target/**" ~/projects/my-rust-code/

./steve-lint.exe --fix --verbose H:\2020_2021\
```
