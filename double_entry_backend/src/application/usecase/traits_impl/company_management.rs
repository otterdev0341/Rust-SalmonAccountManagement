use rocket::async_trait;


#[async_trait]
pub trait CompanyManagement{
    async fn create_company();
    async fn edit_company();
    async fn delete_company();
    
    async fn add_user_to_company();
    async fn remove_user_from_company();

    async fn view_company();
    async fn view_companys();
    async fn view_company_users();
}