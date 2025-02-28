package route_test.auth.sign_up.invalid

import dto.auth.SignUpDtoHelper
import groovy.json.JsonOutput
import io.qameta.allure.Epic
import io.qameta.allure.Step
import spock.lang.Shared
import spock.lang.Specification
import spock.lang.Tag
import utility.AuthCase
import utility.TestRouteManagement
import static io.restassured.RestAssured.*

@Tag("Auth")
@Epic("Test with empty field")
class CreateWithEmptyField extends Specification {

    @Shared
    def target_url

    def setupSpec() {
        target_url = new TestRouteManagement().get_auth_route(AuthCase.SIGN_UP)
    }

    @Step("test create user with empty firstname")
    def "create with empty firstName"(){
        given: "initial value"
        def test_user = new SignUpDtoHelper().get_empty_first_name_field()
        def json_test_user = JsonOutput.toJson(test_user)
        when: "make api call"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_test_user)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()

        then: "extract value"
        def expect_response = 400
        expect: "compare result from api and expect value"
        response.statusCode() == expect_response
        response.statusCode() != 200
        response.statusCode() !=404
        response.statusCode() != 201
    }

    @Step("test create user with empty lastname")
    def "test create user with empty lastname"() {
        given: "initial value"
        def target_user = new SignUpDtoHelper().get_empty_last_name_field()
        def json_target = JsonOutput.toJson(target_user)
        when: "make api call"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_target)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()
        then: "extract value"
        def expect_status = 400
        expect: "compare result from api and expect value"
        response.statusCode() == expect_status
        response.statusCode() != 200
        response.statusCode() !=404
        response.statusCode() != 201
    }

    @Step("test create user with empty email")
    def "test create user with empty email"() {
        given: "initial value"
            def target_user = new SignUpDtoHelper().get_empty_email_field()
            def json_target = JsonOutput.toJson(target_user)
        when: "make api call"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_target)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()
        then: "extract value"
            def expect_status = 400
        expect: "compare result from api and expect value"
            response.statusCode() == expect_status
            response.statusCode() != 200
            response.statusCode() !=404
            response.statusCode() != 201
    }

    @Step( "test create user with empty password")
    def "test create user with empty password"(){
        given: "initial value"
        def target_user = new SignUpDtoHelper().get_empty_password_field()
        def json_target = JsonOutput.toJson(target_user)
        when: "make api call"
            def response =
                    given()
                        .contentType("application/json")
                        .body(json_target)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()
        then: "extract value"
            def expect_status = 400
        expect: "compare result from api and expect value"
            response.statusCode() == expect_status
            response.statusCode() != 200
            response.statusCode() !=404
            response.statusCode() != 201
    }

    }
