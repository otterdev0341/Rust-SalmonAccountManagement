use thiserror::Error;



#[derive(Error, Debug)]
pub enum CompanyError {
    #[error("Invalid company ID or user ID")]
    InvalidId,

    #[error("Forbidden: You don't have permission to perform this action")]
    Forbidden,

    #[error("Company or user not found")]
    NotFound,

    #[error("User already added to this company")]
    UserAlreadyAdded,

    #[error("User already removed from this company")]
    UserAlreadyRemoved,

    #[error("Company creation failed due to invalid name or description")]
    InvalidCompanyData,

    #[error("Internal server error")]
    InternalError,
    
}




#[derive(Error, Debug)]
pub enum CompanySuccess {
    #[error("User added to company")]
    UserAdded,

    #[error("User removed from company")]
    UserRemoved,

    #[error("Company created successfully")]
    CompanyCreated,
    #[error("Company updated successfully")]
    UpdateSuccess,
    #[error("Company deleted successfully")]
    DeleteSuccess,
}


