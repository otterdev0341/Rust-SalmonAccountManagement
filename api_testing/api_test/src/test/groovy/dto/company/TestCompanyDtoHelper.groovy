package dto.company

import utility.HelpRandomNum

class TestCompanyDtoHelper {

    def get_always_new_company_dto(){
        def random = new HelpRandomNum().get_random_num()
        return new ReqCreateCompanyDto(
                name: "test_company${random}",
                description: "this is test company${random}"
        )
    }
}
