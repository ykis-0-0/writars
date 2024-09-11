# wriTER - OF tAR aRCHIVES - IN rUsT
*(oops got the capitalization wrong)*

## tl;dr
```console
[vscode@devcontainer: /workspaces/wri-ta-rs] $ cargo run -- --help
Usage: wri-ta-rs [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Descriptor file path, use "-" for /dev/stdin
  -o, --output <OUTPUT>  Archive output path, use "-" for /dev/stdout
  -f, --force
  -h, --help             Print help
```

## Input format
[Check Here](./src/cli/specimen/extensive.ron)

## Output format
A `.tar` archive, what else do you want?
