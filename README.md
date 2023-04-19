# stsearch

## Getting started

* To install all dependencies and add `stsearch` to your PATH:  `cargo install --path=.`
* Running `stsearch --help` in terminal should produce the following output:
```
Usage: stsearch <LANGUAGE> <QUERY> <FILE>

Arguments:
  <LANGUAGE>  [possible values: semgrep, javascript]
  <QUERY>
  <FILE>

Options:
  -h, --help     Print help
  -V, --version  Print version
```
* Run `stsearch js '$_ + $_' <(<<<'let a = 1 + 1')` should produce the following output:

```
/dev/fd/11:1:9-1:14
/dev/fd/11:1:9-1:14
```

