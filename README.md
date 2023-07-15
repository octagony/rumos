# ☀️ Rumos

CLI utility for controlling screen brightness

## Installation

Install my-project with npm

```bash
Clone repository
    git clone https://github.com/octagony/rumos.git

Build project
    cargo build --release

Find the executable file in target/release
```

## Usage

```bash
Usage: rumos [OPTIONS] <COMMAND>

Commands:
  get   Get brightness level (in percent)
  set   Set brightness level (in percent)
  inc   Increase brightness level (in percent)
  dec   Decrease brightness level (in percent)
  max   Set maximum brightness level
  min   Set mininum brightness level
  help  Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet    Do not output result to console
  -p, --percent  Print only brightness level(percentage)
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

- Get brightness level in a percentage

  ```bash
  rumos -p get
  // 100%
  ```

- Set the maximum brightness level quietly
  ```bash
  rumos max -q
  // No output
  ```
- Decrease and display only brightness level in a percentage
  ```bash
  rumos -p dec 10
  // 90%
  ```
