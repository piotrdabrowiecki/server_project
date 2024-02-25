use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "Not Found",
        }

    }

}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }


}









