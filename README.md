## Databases

This workshop makes use of SQLite databases. As a result, you'll need to have
`sqlite3` and its headers installed:

  * **OS X:** `brew install sqlite`
  * **Debian/Ubuntu:** `apt-get install libsqlite3-dev`
  * **Arch:** `pacman -S sqlite`
  * **Windows:** `vcpkg install sqlite3:x64-windows`

You might also have success (on all platforms) with adding the following to the
`[dependencies]` section in a `Cargo.toml` of a project that uses sqlite:

```toml
libsqlite3-sys = { version = "0.9", features = [ "bundled" ] }
```

## Useful Commands

  * `cargo test` - run all tests in all crates
  * `cargo test -p ${name}` - run tests for crate ${name}
  * `cargo check` - check all crates
  * `cargo test -p ${name}` - run tests for crate ${name}
