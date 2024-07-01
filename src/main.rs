pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod utils;
mod pg;

use utils::cli::run;

fn main() -> Result<()> {
    run()?;
    Ok(())
}
