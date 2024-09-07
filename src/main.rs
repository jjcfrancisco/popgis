mod error;

pub use self::error::{Error, Result};

mod utils;
mod pg;
mod format;

use utils::cli::run;

fn main() -> Result<()> {
    run()?;
    Ok(())
}
