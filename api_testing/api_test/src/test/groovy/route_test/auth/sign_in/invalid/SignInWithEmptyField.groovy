package route_test.auth.sign_in.invalid

import dto.auth.SignInDtoHelper
import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import io.qameta.allure.Epic
import io.qameta.allure.Feature
import io.qameta.allure.Step
import spock.lang.Shared
import spock.lang.Specification
import utility.RouteCase
import utility.TestRouteManagement
import static io.restassured.RestAssured.*

@Feature("Test with some empty field")
class SignInWithEmptyField extends Specification {

    @Shared
    def base_url

    def setupSpec(){
        base_url = new TestRouteManagement().get_route(RouteCase.SIGN_IN)
    }

    @Step("test with valid email with empty password")
    def "test with empty email"(){
        given: "initial value"
        def target_user = new SignInDtoHelper().case_5_valid_email_empty_password()
        def json_target = JsonOutput.toJson(target_user)
        when:
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_target)
                    .when()
                        .post(base_url)
                    .then()
                        .extract()
                        .response()
        then:
            def expect_status = 400
            def json_response = new JsonSlurper().parseText(response.asString())
            println json_response
        expect:
            response.statusCode() == expect_status
    }

    @Step("test with empty password with empty email")
    def "test with empty password"() {
        given: "initial value"
        def target_user = new SignInDtoHelper().case_6_valid_password_empty_email()
        def json_target = JsonOutput.toJson(target_user)
        when:
        def response =
                given()
                        .contentType("application/json")
                        .body(json_target)
                        .when()
                        .post(base_url)
                        .then()
                        .extract()
                        .response()
        then:
        def expect_status = 400
        def json_response = new JsonSlurper().parseText(response.asString())
        println json_response
        expect:
        response.statusCode() == expect_status
    }

    @Step("test with empty password and email")
    def"test with empty password and email"() {
        given: "initial value"
        def target_user = new SignInDtoHelper().case_7_empty_email_empty_password()
        def json_target = JsonOutput.toJson(target_user)
        when:
        def response =
                given()
                        .contentType("application/json")
                        .body(json_target)
                        .when()
                        .post(base_url)
                        .then()
                        .extract()
                        .response()
        then:
        def expect_status = 400
        def json_response = new JsonSlurper().parseText(response.asString())
        println json_response
        expect:
        response.statusCode() == expect_status
    }



}