use rocket::async_trait;


#[async_trait]
pub trait ImplProjectManagement {
    async fn create_project();
    async fn edit_project();
    async fn delete_project();
    async fn view_project();
    async fn view_projects();

    async fn create_project_location();
    async fn edit_project_location();
    async fn delete_project_location();
    async fn view_project_location();

    async fn create_location_type();
    async fn edit_location_type();
    async fn delete_location_type();
    async fn view_location_type();

    async fn attach_location_to_project();
    async fn detach_location_from_project();

    async fn create_project_info();
    async fn edit_project_info();
    async fn delete_project_info();
    async fn view_project_info();
    async fn view_project_infos();

    async fn create_project_status();
    async fn edit_project_status();
    async fn delete_project_status();
    async fn view_project_status();
    async fn view_project_statuses();

}