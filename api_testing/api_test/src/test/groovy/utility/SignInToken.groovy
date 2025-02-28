package utility

import dto.auth.SignInDtoHelper
import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import io.qameta.allure.Description
import io.qameta.allure.Epic
import spock.lang.Shared
import spock.lang.Specification
import spock.lang.Subject

import static io.restassured.RestAssured.expect
import static io.restassured.RestAssured.given

class SignInToken {

    String get_valid_token(){
        def base_url = new TestRouteManagement().get_route(RouteCase.SIGN_IN)
        def target_user = new SignInDtoHelper().case_1_valid_email_valid_password()
        def json_target = JsonOutput.toJson(target_user)

        def response =
                given()
                    .contentType("application/json")
                    .body(json_target)
                .when()
                    .post(base_url)
                .then()
                    .extract()
                    .response()
        def json_response = new JsonSlurper().parseText(response.asString())
        def token = json_response.data.token
        return token
    }
}


@Epic("test internal function")
class TestSignInToken extends Specification {

    @Description("this function will return valid token to use for other test, that need authenticate")
    def "this must return valid token"(){
        @Subject
        def token = new SignInToken().get_valid_token()

        expect:
            token != null

    }
}
