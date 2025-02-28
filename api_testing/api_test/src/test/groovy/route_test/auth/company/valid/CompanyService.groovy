package route_test.auth.company.valid

import dto.company.ReqUpdateCompanyDto
import groovy.json.JsonSlurper
import utility.CompanyCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.*
import dto.company.ReqCreateCompanyDto

class CompanyService {

    Optional<String> create_company(ReqCreateCompanyDto new_company, String user_token) {
        def create_company_url = new TestRouteManagement().get_company_route(CompanyCase.CREATE_COMPANY)
        def response =
                given()
                    .contentType("application/json")
                    .header("Authorization", "Bearer ${user_token}")
                    .body(new_company)
                .when()
                    .post(create_company_url)
                .then()
                    .extract()
                    .response()

        def json_response = new JsonSlurper().parseText(response.asString())
        def response_data = json_response.data
        String created_id = response_data.idCreated
        return Optional.ofNullable(created_id)
    }

    Optional<String> view_company(String user_token, String company_id){
        
    }
}
