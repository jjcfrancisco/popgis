use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- utils
    FailedInputValidation(String),
    Mode(String),

    // -- pg
    TableExists(String),
    CannotAppend(String),

    // -- file_types
    UnsupportedFileExtension(String),
    UnsupportedShapeType(String),
    ContainsMixedDataTypes(String),

    // -- Externals
    #[from]
    Io(std::io::Error),
    #[from]
    Pg(postgres::Error),
    #[from]
    Shapefile(shapefile::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
