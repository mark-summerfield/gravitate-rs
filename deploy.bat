SET CARGO_TARGET_DIR=C:\tmp\targets\gravitate
mkdir %CARGO_TARGET_DIR%
cargo build --release -- %*
copy /Y %CARGO_TARGET_DIR%\release\gravitate.exe .
rcedit gravitate.exe --set-icon images\gravitate.ico
copy /Y gravitate.exe C:\bin
