pub mod dec_1;
pub mod dec_2;

pub type AOCResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct CustomError(pub String);

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}