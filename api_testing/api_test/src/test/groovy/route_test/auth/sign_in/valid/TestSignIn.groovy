package route_test.auth.sign_in.valid

import dto.auth.SignInDtoHelper
import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import io.qameta.allure.Epic
import io.qameta.allure.Step
import spock.lang.Shared
import spock.lang.Specification
import utility.RouteCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.*

@Epic("Test Sign In")
class TestSignIn extends Specification {

    @Shared
    def base_url


    def setupSpec(){
        base_url = new TestRouteManagement().get_route(RouteCase.SIGN_IN)
    }

    @Step("Sign in with valid data")
    def "test_sign_in"(){
        given: "initial value"
            def target_user = new SignInDtoHelper().case_1_valid_email_valid_password()
            def json_target = JsonOutput.toJson(target_user)
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
            def expect_status = 200

        def json_response = new JsonSlurper().parseText(response.asString())
        def token = json_response.data.token

        expect: "compare result from api and expect value"
            response.statusCode() == expect_status
            response.statusCode() != 400
            response.statusCode() != 500
            token != null
    }




}