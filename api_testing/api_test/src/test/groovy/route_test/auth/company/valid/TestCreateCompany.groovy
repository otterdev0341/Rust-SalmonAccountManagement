package route_test.auth.company.valid

import groovy.json.JsonSlurper
import spock.lang.Shared
import utility.CompanyCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.*
import dto.auth.SignInDtoHelper
import dto.company.TestCompanyDtoHelper
import groovy.json.JsonOutput
import spock.lang.Specification
import utility.SignInToken

class TestCreateCompany extends Specification {

    @Shared
    def always_new_company_id

    @Shared
    def lastest_view_count

    @Shared
    def bearer_token

    def setupSpec(){
        bearer_token = new SignInToken().get_valid_token()
    }

    def "Test Create Company"() {
        given:
        def new_company = new TestCompanyDtoHelper().get_always_new_company_dto()
        def json_company = JsonOutput.toJson(new_company)

        def create_company_url = new TestRouteManagement().get_company_route(CompanyCase.CREATE_COMPANY, "")
        when:
        def response =
                given()
                    .contentType("application/json")
                    .header("Authorization", "Bearer ${bearer_token}")
                    .body(json_company)
                .when()
                    .post(create_company_url)
                .then()
                    .extract()
                    .response()

        def json_response = new JsonSlurper().parseText(response.asString())
        def response_data = json_response.data
        def created_id = response_data.idCreated

        always_new_company_id = created_id
        then:
        def expect_status = 201
        expect:
        response.statusCode() == expect_status
        response.statusCode() != 200
        response.statusCode() != 400
        response.statusCode() != 404


    }

    def "View company by id"() {
        given:
        def bearer_token = new SignInToken().get_valid_token()
        def view_company_url = new TestRouteManagement().get_company_route(CompanyCase.VIEW_COMPANY, always_new_company_id)

        when: ""

        then: ""
        expect: ""
    }
}
