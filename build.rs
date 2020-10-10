#[cfg(target_env = "msvc")]
use vcpkg;

fn main() {
    #[cfg(target_env = "msvc")]
    {
        println!("cargo:rerun-if-env-changed=VCPKGRS_DYNAMIC");
        // **Note:** You must set one of `RUSTFLAGS=-Ctarget-feature=+crt-static` or
        // `VCPKGRS_DYNAMIC=1` in your environment or the vcpkg-rs helper
        // will not find any libraries. If `VCPKGRS_DYNAMIC` is set, `cargo install` will
        // generate dynamically linked binaries, in which case you will have to arrange for
        // dlls from your Vcpkg installation to be available in your path.
        let p = vcpkg::find_package("libssh").unwrap();
        if p.is_static {
            println!("cargo:rustc-flags=-l Shell32");
            println!("cargo:rustc-flags=-l static=ssh");
        }
    }
    #[cfg(not(target_env = "msvc"))]
    {
        println!("cargo:rustc-flags=-l dylib=ssh");
    }
}
