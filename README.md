# Gravitate

A SameGame/TileFall-like game written in Rust/FLTK.

Tested on Linux and Windows.

<div align="center">

![Screenshot](screenshot.png)

</div>

`gravitate.exe` *is a precompiled Windows binary that should run on any
64-bit version of Windows* (1.6MB; MD5 cbe6e0c9d269d23b3bd374ee46239078)

## Dependencies

```toml
[dependencies]
chrono = "^0.4"
dirs = "^4"
num = "^0.4"
rust-ini = "^0.18"
rustc_version_runtime = "^0.2"
state = "^0.5"
thousands = "^0.2.0"

[dependencies.fltk]
version = "^1.3"
features = [ "no-pango",]
git = "https://github.com/fltk-rs/fltk-rs"

[dependencies.rand]
version = "^0.8"
features = [ "alloc",]
```

## License

This project is licensed under the `GPL-3.0` license. The GPL-3.0 license is a copyleft license that ensures software remains free and open source. It grants the freedom to distribute and modify the software, while requiring the availability of source code. Derivative works can be created and distributed under the same terms. It is compatible with other GNU licenses and allows for commercial use. The license includes a patent grant and requires a license notice when distributing the software. For full details and obligations, please refer to the GPL-3.0 license text.

## Other Versions

For versions in Tcl/Tk, C++/wxWidgets, D/GtkD, Nim/NiGui, Java/AWT/Swing,
Python/Tkinter, Python/wxPython, and JavaScript see
[www.qtrac.eu/gravitate.html](http://www.qtrac.eu/gravitate.html).
