Apex Legends Offset Dumper
==========================

The `r5apex.exe` binary on disk is encrypted and must be dumped from memory first.

From there you can dump its offsets with `cargo run --release -- "r5apex.bin" ini > stdout.ini`. The script will analyze the dumped binary and extract its offsets. Use these offsets to update the cheat. You can get human readable output without offsets with `cargo run --release -- "r5apex.exe" human > stdout.md`.

Alternatively use the precompiled wasm version: <https://casualhacks.net/apexdream/apexdumper.html>. Just drop the `r5apex.exe` on the file input and wait.

Publish the WebApp
------------------

```
cargo build --release --lib --target=wasm32-unknown-unknown
copy /y /b "target\wasm32-unknown-unknown\release\libapexdumper.wasm" "apexdumper.wasm"
```
