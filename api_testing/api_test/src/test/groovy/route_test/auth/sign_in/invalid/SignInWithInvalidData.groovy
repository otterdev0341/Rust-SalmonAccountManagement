package route_test.auth.sign_in.invalid

import dto.auth.SignInDtoHelper
import groovy.json.JsonOutput
import io.qameta.allure.Feature
import spock.lang.Shared
import spock.lang.Specification
import utility.AuthCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.given

@Feature("Test sign-in with invalid value")
class SignInWithInvalidData extends Specification {

    @Shared
    def base_url

    @Shared
    def help_sign_in_dto

    def setupSpec() {
        base_url = new TestRouteManagement().get_auth_route(AuthCase.SIGN_IN)
        help_sign_in_dto = new SignInDtoHelper()
    }

    def "case_2_invalid_email_and_password"() {
        given: "initial value"
        def target_user = help_sign_in_dto.case_2_invalid_email_and_password()
        def target_json = JsonOutput.toJson(target_user)
        when: "call api"
        def response =
                given()
                        .contentType("application/json")
                        .body(target_json)
                        .when()
                        .post(base_url)
                        .then()
                        .extract()
                        .response()
        then: "extract value"
        def expect_status = 401
        expect:
        response.statusCode() == expect_status
    }


    def "case_3_invalid_email_valid_password"() {
        given: "initial value"
            def test_target = new SignInDtoHelper().case_3_invalid_email_valid_password()
            def json_target = JsonOutput.toJson(test_target)
        when: "call api"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_target)
                    .when()
                        .post(base_url)
                    .then()
                        .extract()
                        .response()

        then: "extract value"
            def expect_status = 401

        expect: "compare result from api and expect value"
            response.statusCode() == expect_status
            response.statusCode() != 200
            response.statusCode() != 404
    }

    def "case_4_valid_email_invalid_password"() {
        given: "initial value"
        def test_target = new SignInDtoHelper().case_4_valid_email_invalid_password()
        def json_target = JsonOutput.toJson(test_target)
        when: "call api"
        def response =
                given()
                        .contentType("application/json")
                        .body(json_target)
                        .when()
                        .post(base_url)
                        .then()
                        .extract()
                        .response()

        then: "extract value"
        def expect_status = 401

        expect: "compare result from api and expect value"
        response.statusCode() == expect_status
        response.statusCode() != 200
        response.statusCode() != 404
    }
}