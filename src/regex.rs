#[cfg(feature = "full-regex")]
pub use regex::Regex;

#[cfg(feature = "lite-regex")]
pub use regex_lite::Regex;

#[cfg(not(any(feature = "full-regex", feature = "lite-regex")))]
compile_error!("Either 'full-regex' or 'lite-regex' feature must be enabled");
