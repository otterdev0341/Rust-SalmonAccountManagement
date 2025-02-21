use rocket::async_trait;





#[async_trait]
pub trait ImplUserManagement{
    async fn forgot_password() -> &'static str;
    async fn delete_account() -> &'static str;
    async fn change_email() -> &'static str;
    async fn change_password() -> &'static str;
    
}