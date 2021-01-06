
fn main() {
    let dst = cmake::Config::new("tvision")
                .define("CMAKE_BUILD_TYPE", "Release")
                .build();

    //pkg_config::Config::new()
    //    .atleast_version("6.1")
    //    .probe("ncursesw")
    //    .expect("Unable to find 'ncursesw' library");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=tvision");
    println!("cargo:rustc-link-lib=stdc++");
    //println!("cargo:rustc-link-lib=static=ncurses");
}
