#[cfg(target_env = "msvc")]
use vcpkg;

fn main() {
    #[cfg(target_env = "msvc")]
    {
        // **Note:** You must set one of `RUSTFLAGS=-Ctarget-feature=+crt-static` or
        // `VCPKGRS_DYNAMIC=1` in your environment or the vcpkg-rs helper
        // will not find any libraries. If `VCPKGRS_DYNAMIC` is set, `cargo install` will
        // generate dynamically linked binaries, in which case you will have to arrange for
        // dlls from your Vcpkg installation to be available in your path.
        vcpkg::Config::new()
            .emit_includes(true)
            .probe("ssh")
            .unwrap();
    }
    #[cfg(not(target_env = "msvc"))]
    {
        println!("cargo:rustc-flags=-l dylib=ssh");
    }
}
