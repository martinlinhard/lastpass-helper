# lastpass-helper
This is a simple tool for connecting to LastPass using the lastpass cli.

## Installation
### Prerequisites
* Install the lastpass cli (https://github.com/lastpass/lastpass-cli)
* Install Rust (https://www.rust-lang.org/tools/install)
* Install Rofi (https://github.com/davatorium/rofi)
* Sign in with lastpass (lpass login *username*)

### Actual installation
```bash
git clone https://github.com/realSuffix/lastpass-helper
cargo install --force --path lastpass-helper
```

## Usage
To use the application, run it **either** with the -u flag (to copy the username) or with the -p flag (to copy the password).
