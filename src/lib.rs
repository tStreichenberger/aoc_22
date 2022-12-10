pub mod dec_1;
pub mod dec_2;
pub mod dec_3;
pub mod dec_4;
pub mod dec_5;
pub mod dec_6;
pub mod dec_7;
pub mod dec_8;
pub mod dec_9;
pub mod dec_10;

pub type AOCError = Box<dyn std::error::Error>;
pub type AOCResult<T> = Result<T, AOCError>;

#[derive(Debug)]
pub struct CustomError(pub String);

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod io {
    use crate::AOCResult;
    use std::fs;
    pub fn read_in(day: u8) -> AOCResult<String> {
        let input = fs::read_to_string(format!("IO/dec_{day}/in.txt"))?;
        Ok(input)
    }

    pub fn write_out<C: ToString>(day: u8, sol_num: u8, content: C) -> AOCResult<()> {
        fs::write(
            format!("IO/dec_{day}/out_{sol_num}.txt"),
            content.to_string(),
        )?;
        Ok(())
    }
}
