# Edge-Fixer

_A program that replaces Microsoft Edge and Bing in the start menu search._ \
_Currently, with Firefox and DuckDuckGo (in the future, it will be configurable)._

# Install

1. Download the newest EXE from the [release section](https://gitlab.com/Captainpast/edge-fixer/-/releases).
2. Move it to a folder of your choice (do not move the EXE after execution).
3. Run the EXE via double-click. There will be no response.

# Uninstall

1. Navigate to your chosen folder for the EXE.
2. Open the advanced context menu via `shift + right-click` and click _Open CMD/Powershell here_.
3. Type `./edge-fixer.exe uninstall` and enter. There should be positive feedback.

# Build
## On Debian (Ubuntu, Mint and others)

### Build
```bash
# install new rust toolchain
$ rustup target add x86_64-pc-windows-gnu
# build the exe
$ cargo build --target=x86_64-pc-windows-gnu --release
```

### Signing
```bash
# install dependencies
$ sudo apt install openssl osslsigncode
# create signing certificate (if you dont own one)
$ openssl req -x509 -newkey rsa:4096 -keyout cert/key.pem -out cert/cert.pem -sha256 -days 365
# signing the exe
$ osslsigncode sign -certs cert/cert.pem -key cert/key.pem \
    -n "Edge Fixer" -i https://gitlab.com/Captainpast \
    -t http://timestamp.digicert.com \
    -in target/x86_64-pc-windows-gnu/release/edge-fixer.exe -out edge-fixer-signed.exe
```