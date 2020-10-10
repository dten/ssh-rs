# ssh
Fork of pijul.org/ssh (dead link)

The crate at https://crates.io/crates/ssh points to dead source, this is a fork with updates

# Windows and vcpkg
Requires vcpkg installed and VCPKG_ROOT set.

## Static linking
```
vcpkg.exe install libssh:x64-windows-static
$env:RUSTFLAGS="-Ctarget-feature=+crt-static"
cargo build
```
## Dynamic linking (make sure to distribute require DLLs)
```
vcpkg.exe install libssh:x64-windows
$env:VCPKGRS_DYNAMIC=1
cargo build
```