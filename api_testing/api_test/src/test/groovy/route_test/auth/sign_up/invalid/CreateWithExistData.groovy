package route_test.auth.sign_up.invalid

import dto.auth.SignUpDtoHelper
import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import io.qameta.allure.Epic
import io.qameta.allure.Step
import spock.lang.Shared
import spock.lang.Specification
import spock.lang.Tag
import utility.RouteCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.*

@Tag("Auth")
@Epic("Test with exist user data")
class CreateWithExistData extends Specification {

    @Shared
    def target_url

    def setupSpec() {
        def helper = new TestRouteManagement()
        target_url = helper.get_route(RouteCase.SIGN_UP)
    }

    @Step("create with exist data")
    def "create with exist data"() {
        given: "initial value"

        def target_user = new SignUpDtoHelper().get_exist_user_data()
        def json_target = JsonOutput.toJson(target_user)
        when: "call api"
        def response =
                given()
                    .contentType("application/json")
                    .body(json_target)
                .when()
                    .post(target_url)
                .then()
                    .extract()
                    .response()
        then:
        def expect_status = 400
        expect:
        response.statusCode() == expect_status
        response.statusCode() != 200
        response.statusCode() != 404
        response.statusCode() != 201
    }

    @Step("create with exist email")
    def "create with exist email"(){
        given: "initial value"
            def target_user = new SignUpDtoHelper().get_exist_user_email()
            def json_target = JsonOutput.toJson(target_user)
        when: "call api"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_target)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()
        then: "extract"
            def expect_status = 400
            def expect_message = "Email already exists"

            def parse_data = new JsonSlurper().parseText(response.asString())

        expect: "compare"
            response.statusCode() != 200
            response.statusCode() != 404
            response.statusCode() != 201
            response.statusCode() == expect_status
            parse_data.message == expect_message

    }

    @Step("create with exist username")
    def "create with exist username"(){
        given: "initial value"
            def target_user = new SignUpDtoHelper().get_exist_username_field()
            def json_user = JsonOutput.toJson(target_user)
        when: "call api"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_user)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()
        then: "extract"
            def expect_status = 400
            def expect_message = "User already exists"

            def parse_data = new JsonSlurper().parseText(response.asString())

        expect: "compare"
            response.statusCode() != 200
            response.statusCode() != 404
            response.statusCode() != 201
            response.statusCode() == expect_status
            parse_data.message == expect_message
    }
}
