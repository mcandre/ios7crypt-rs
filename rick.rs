//! Build configuration

extern crate tinyrick;
extern crate zip;
extern crate glob;

use std::io;
use std::fs;
use std::path;

static VERSION : &str = "0.0.5";

/// Run clippy
fn clippy() {
  tinyrick::exec!("cargo", &["clippy"]);
}

/// Run linters
fn lint() {
  tinyrick::deps(clippy);
}

/// Compile project
fn build() {
  tinyrick::exec!("cargo", &["build"]);
}

/// Generate documentation
fn doc() {
  tinyrick::exec!("cargo", &["doc"]);
}

/// Install applications
fn install_binaries() {
  tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Install artifacts
fn install() {
  tinyrick::deps(install_binaries);
}

/// Uninstall artifacts
fn uninstall() {
  tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run unit tests
fn unit_test() {
  tinyrick::exec!("cargo", &["test"]);
}

/// Run integration tests
fn integration_test() {
  tinyrick::deps(install);

  tinyrick::exec!("ios7crypt", &["-e", "monkey"]);
  assert!(tinyrick::exec_stdout_utf8!("ios7crypt", &["-d", "060b002f474b10"]) == "monkey\n");
}

/// Run all tests
fn test() {
  tinyrick::deps(unit_test);
  tinyrick::deps(integration_test);
}

/// Generate and archive ports
fn port() {
  let architectures : &[&str] = &["x86_64", "i686"];
  let libcs : &[&str] = &["gnu", "musl"];

  for architecture in architectures {
    for libc in libcs {
      tinyrick::exec!("sh", &["crosscompile-linux.sh", architecture, libc]);
    }
  }

  let archive_path : &str = &format!("ios7crypt-{}.zip", VERSION);

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
  tinyrick::exec!("cargo", &["publish"]);
}

/// Delete archives
fn clean_archives() {
  for p in glob::glob("*.zip").unwrap() {
    fs::remove_file(p.unwrap()).unwrap();
  }
}

/// Run cargo clean
fn clean_cargo() {
  tinyrick::exec!("cargo", &["clean"]);
}

/// Clean workspaces
fn clean() {
  tinyrick::deps(clean_archives);
  tinyrick::deps(clean_cargo);
}

tinyrick::wubba_lubba_dub_dub!(
  test;
  clippy,
  lint,
  build,
  doc,
  install_binaries,
  install,
  uninstall,
  unit_test,
  integration_test,
  port,
  publish,
  clean_cargo,
  clean
);
