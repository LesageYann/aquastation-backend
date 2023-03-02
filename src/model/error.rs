pub enum  ASError {
    Unauthorized,
    NotFound,
    InvalidRequest(Vec<String>), // ()
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InternalError
}