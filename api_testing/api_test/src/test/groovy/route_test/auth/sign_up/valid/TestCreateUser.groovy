package route_test.auth.sign_up.valid

import dto.auth.SignUpDtoHelper
import dto.auth.ReqCreateUserDto
import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import io.qameta.allure.Description
import io.qameta.allure.Epic
import spock.lang.Specification
import utility.enum_type.AuthCase
import utility.TestRouteManagement

import static io.restassured.RestAssured.*

@Epic("Test Create user")
class TestCreateUser extends Specification{

    @Description("This must be pass")
    def "test_create_user"() {
        given: "create new user to persisted"
            // get new data
            ReqCreateUserDto new_user = new SignUpDtoHelper().get_always_new_user()
            // convert to json to use in api
            def new_user_json = JsonOutput.toJson(new_user)
            def target_url = new TestRouteManagement().get_auth_route(AuthCase.SIGN_UP)
        when: "try to connect to api"
            def response
                        = given().contentType("application/json")
                            .body(new_user_json)
                        .when()
                            .post(target_url)
                        .then()
                            .extract()
                            .response()
            def json_response = new JsonSlurper().parseText(response.asString())
        then: "check response and validate result"

        response.statusCode() == 201
        response.statusCode() != 400
        response.statusCode() != 200
        response.statusCode() != 404

    }

    @Description("create default user")
    def "create_default_user"(){
        given:
        def kotaro_user = new ReqCreateUserDto(
                firstName: "kotaro",
                lastName: "river_otter",
                email: "kotaro@work.com",
                username: "kotaro_cute",
                password: "kotaro1235555")

        def kotaro_json = JsonOutput.toJson(kotaro_user)
        def target_url = new TestRouteManagement().get_auth_route(AuthCase.SIGN_UP)
        when:
        def response
                    = given().contentType("application/json")
                        .body(kotaro_json)
                    .when()
                        .post(target_url)
                    .then()
                        .extract()
                        .response()
        def json_response = new JsonSlurper().parseText(response.asString())
        then:
        response.statusCode() == 201 || response.statusCode() == 400

    }
}
