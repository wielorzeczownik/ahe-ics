//! Embeds the application icon into the Windows executables

fn main() {
  println!("cargo:rerun-if-changed=assets/icon.ico");
  embed_windows_icon();
}

#[cfg(windows)]
fn embed_windows_icon() {
  if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
    return;
  }

  let mut resource = winresource::WindowsResource::new();
  resource.set_icon("assets/icon.ico");

  if let Err(error) = resource.compile() {
    println!("cargo:warning=could not embed the Windows icon: {error}");
  }
}

#[cfg(not(windows))]
fn embed_windows_icon() {}
