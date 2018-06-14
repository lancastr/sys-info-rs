extern crate cc;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let target_os = target.split('-').nth(2).unwrap();

    let mut builder = cc::Build::new();
    match target_os {
        "linux" | "android" => builder.file("c/linux.c"),
        "freebsd" => builder.file("c/freebsd.c"),
        "macos" | "ios" => builder.file("c/macos.c"),
        "windows" => {
            println!("cargo:rustc-flags=-l psapi");
            builder.file("c/windows.c")
        },
        _ => panic!("Unsupported system: {}", target_os)
    };
    builder.compile("info");
}