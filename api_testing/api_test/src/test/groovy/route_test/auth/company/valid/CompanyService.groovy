package route_test.auth.company.valid

import dto.company.ReqUpdateCompanyDto
import groovy.json.JsonOutput
import io.restassured.response.Response
import utility.enum_type.CompanyCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.*
import dto.company.ReqCreateCompanyDto

class CompanyService {

    def inject_route = new TestRouteManagement()

    Optional<Response> create_company(ReqCreateCompanyDto new_company, String user_token) {
        def create_company_url = inject_route.get_company_route(CompanyCase.CREATE_COMPANY)
        def json_new_company = JsonOutput.toJson(new_company)
        def response =
                given()
                    .contentType("application/json")
                    .header("Authorization", "Bearer ${user_token}")
                    .body(json_new_company)
                .when()
                    .post(create_company_url)
                .then()
                    .extract()
                    .response()

        return Optional.of(response)
    }

    Optional<Response> view_company(String user_token, String company_id){
        def view_company_url = inject_route.get_company_route(CompanyCase.VIEW_COMPANY)

        def response =
                given()
                    .contentType("application/json")
                    .header("Authorization", "Bearer ${user_token}")
                .when()
                    .get("${view_company_url}/${company_id}")
                .then()
                    .extract()
                    .response()

        return Optional.of(response)
    }

    Optional<Response> view_companies(String user_token){
        def view_companies_url = inject_route.get_company_route(CompanyCase.VIEW_COMPANIES)

        def response =
                    given()
                        .contentType("application/json")
                        .header("Authorization", "Bearer ${user_token}")
                    .when()
                        .get("${view_companies_url}")
                    .then()
                        .extract()
                        .response()
        return Optional.of(response)
    }

    Optional<Response> edit_company(String user_token, String company_id ,ReqUpdateCompanyDto data){
        def edit_company_url = inject_route.get_company_route(CompanyCase.EDIT_COMPANY)
        def json_req_updated_company_dto = JsonOutput.toJson(data)

        def response =
                given()
                    .contentType("application/json")
                    .header("Authorization", "Bearer ${user_token}")
                    .body(json_req_updated_company_dto)
                .when()
                    .put("${edit_company_url}/${company_id}")
                .then()
                    .extract()
                    .response()
        return Optional.of(response)
    }

    Optional<Response> delete_company(String user_token, String company_id){
        def del_company_url = inject_route.get_company_route(CompanyCase.DELETE_COMPANY)

        def response =
                given()
                    .contentType("application/json")
                    .header("Authorization", "Bearer ${user_token}")
                .when()
                    .delete("${del_company_url}/${company_id}")
                .then()
                    .extract()
                    .response()
        return Optional.of(response)
    }














}
