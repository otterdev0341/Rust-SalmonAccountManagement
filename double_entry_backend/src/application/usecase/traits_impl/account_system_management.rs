use rocket::async_trait;



#[async_trait]
 pub trait AccountSystemManagement {
    fn upload_chart_of_account_template();
    fn download_chart_of_account_template();

    fn create_chart_of_account();
    fn edit_chart_of_account();
    fn delete_chart_of_account();
    fn view_chart_of_account();
    fn view_chart_of_accounts();

    fn view_char_of_account_balance();
    fn view_chart_of_account_transaction();
    fn view_chart_of_account_transactions();

    fn create_bank_account();
    fn edit_bank_account();
    fn delete_bank_account();
    fn view_bank_account();
    fn view_bank_accounts();

    fn create_bank_account_type();
    fn edit_bank_account_type();
    fn delete_bank_account_type();
    fn view_bank_account_type();
    fn view_bank_account_types();

    fn create_jounal_entry();
    fn edit_jounal_entry();
    fn delete_jounal_entry();
    fn view_jounal_entry();
    fn view_jounal_entries();
}