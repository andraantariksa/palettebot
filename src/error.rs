pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    ReqwestError(reqwest::Error),
    ImageError(image::ImageError),
    SerdeJSONError(serde_json::Error),
    OptionNoneError
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::ImageError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJSONError(err)
    }
}
