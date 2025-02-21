use rocket::async_trait;

#[async_trait]
pub trait CustomerManagement {
    // one customer can relate to
    async fn create_customer();
    async fn edit_customer();
    async fn delete_customer();
    async fn view_customer();
    async fn view_customers();

    async fn create_customer_contact();
    async fn edit_customer_contact();
    async fn delete_customer_contact();
    async fn view_customer_contact();


}