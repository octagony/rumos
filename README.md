# ☀️ Rumos

CLI utility for controlling screen brightness

![Demo preview](demo.gif)

## Installation

- Build from source

  ```bash
  Clone repository
      git clone https://github.com/octagony/rumos.git

  Build project
      cargo build --release

  Find the executable file in target/release
  ```

- Use cargo install

  ```bash
  cargo install rumos
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
  list  List all available brightness devices
  help  Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet    Do not output result to console
  -p, --percent  Print only brightness level(percentage)
  -d, --device   Print only brightness level(percentage)
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
- Get list of devices

  ```bash
  rumos list
  // -> intel_backlight (current: 100)
  // -> nvidia (current: 100)
- Set brightness level to target device

  ```bash
  rumos -d intel_backlight set 80
  // intel_backlight: 80
- (Recipe) Use rumos with dunstify.

  You can find a script to control the brightness level [in my DWM config](https://github.com/octagony/dwm-config-files/blob/master/dwm/scripts/brightnessnotifications.sh). In a simplified version you can use this input

  ```bash
  dunstify $(rumos -p get) -t 2000
  ```
