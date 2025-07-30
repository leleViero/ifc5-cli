// re-exporting modules for library functionality
pub mod parser;
pub mod transformer;
pub mod writer;
pub mod schema;
pub mod models;
pub mod app;

pub use app::run;
// re-exporting the `run` function from the `app` module
// This allows users to call use lib::run;