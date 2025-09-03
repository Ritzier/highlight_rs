mod pages;

mod app;
pub use app::App;

#[cfg(feature = "ssr")]
mod shell;
#[cfg(feature = "ssr")]
pub use shell::shell;
