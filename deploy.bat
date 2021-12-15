cargo build --release -- %*
copy /Y target\release\gravitate.exe .
rcedit gravitate.exe --set-icon images\gravitate.ico
copy /Y gravitate.exe C:\bin
