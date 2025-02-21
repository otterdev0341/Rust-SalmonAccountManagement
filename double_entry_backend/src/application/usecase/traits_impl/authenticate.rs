use rocket::async_trait;

#[async_trait]
pub trait ImplAuthenticate {
    async fn register() -> &'static str;
    async fn login() -> &'static str;
    async fn logout() -> &'static str;
}