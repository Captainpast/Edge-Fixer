cargo build --target=x86_64-pc-windows-gnu --release
osslsigncode sign -certs cert/cert.pem -key cert/key.pem \
    -n "Edge Fixer" -i https://gitlab.com/Captainpast \
    -t http://timestamp.digicert.com \
    -in target/x86_64-pc-windows-gnu/release/edge-fixer.exe -out edge-fixer-signed.exe