//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;
extern crate zip;
extern crate glob;

use std::io;
use std::fs;
use std::path;

/// Generate documentation
fn doc() {
    tinyrick_extras::doc();
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
}

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
}

/// Install binaries
fn install() {
    tinyrick_extras::install_binaries();
}

/// Uninstall binaries
fn uninstall() {
    tinyrick_extras::uninstall_binaries();
}

/// Doc, lint, and run tests
fn test() {
    tinyrick::deps(lint);
    tinyrick::deps(install);

    tinyrick_extras::unit_test();

    tinyrick::exec!("ios7crypt", &["-e", "monkey"]);
    assert!(tinyrick::exec_stdout_utf8!("ios7crypt", &["-d", "060b002f474b10"]) == "monkey\n");
    assert!(!tinyrick::exec_status!("ios7crypt").success());
}

/// Doc, lint, test, and compile
fn build() {
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// Generate and archive ports
fn port() {
    let architectures : &[&str] = &["x86_64", "i686"];
    let libcs : &[&str] = &["gnu", "musl"];

    for architecture in architectures {
        for libc in libcs {
            tinyrick::exec!("sh", &["crosscompile-linux", architecture, libc]);
        }
    }

    let archive_path : &str = &format!("{}-{}.zip", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let zip_file : fs::File = fs::File::create(archive_path).unwrap();

    let zip_writer : &mut zip::ZipWriter<fs::File> = &mut zip::ZipWriter::new(zip_file);

    let file_options : zip::write::FileOptions = zip::write::FileOptions::default();

    let target_path : &path::Path = path::Path::new("target");

    for architecture in architectures {
        for libc in libcs {
            let target_with_environment : path::PathBuf = target_path.join(format!("{}-unknown-linux-{}", architecture, libc));

            let binary_path : path::PathBuf = target_with_environment
            .join("release")
            .join("ios7crypt");

            let binary_path_str : &str = binary_path.to_str().unwrap();

            let entry_file : &mut fs::File = &mut fs::File::open(binary_path_str).unwrap();

            zip_writer.start_file(binary_path_str, file_options).unwrap();
            io::copy(entry_file, zip_writer).unwrap();
        }
    }
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Delete archives
fn clean_archives() {
    for p in glob::glob("*.zip").unwrap() {
        fs::remove_file(p.unwrap()).unwrap();
    }
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_archives);
    tinyrick_extras::clean_cargo();
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        doc,
        clippy,
        lint,
        install,
        uninstall,
        test,
        port,
        publish,
        clean_archives,
        clean
    );
}
