package utility

import io.qameta.allure.Epic
import spock.lang.Specification


class TestRouteManagement {
    static String base = "http://127.0.0.1:8000"

    String get_auth_route(AuthCase routeCase) {

        switch (routeCase) {
            case AuthCase.SIGN_UP:
                return base + "/v1/auth/sign-up"
            case AuthCase.SIGN_IN:
                return base + "/v1/auth/sign-in"
            default:
                return ""
        }
    }

    String get_company_route(CompanyCase routeCase, String target_id){
        switch (routeCase) {
            case CompanyCase.CREATE_COMPANY:
                return base + "/v1/company"
            case CompanyCase.VIEW_COMPANY:
                return base + "/v1/company/${target_id}" + target_id
            case CompanyCase.VIEW_COMPANIES:
                return base + "/v1/company"
            case CompanyCase.UPDATE_COMPANY:
                return base + "/v1/company/${target_id}" + target_id
            case CompanyCase.DELETE_COMPANY:
                return base + "/v1/company/${target_id}" + target_id
            default:
                return ""
        }
    }

}

@Epic("Test internal function")
class TestTestRouteManagement extends Specification {

    def "test_function"() {
        given: "an instance of TestRouteManagement"
        TestRouteManagement helper = new TestRouteManagement()
        def criteria = AuthCase.SIGN_UP

        when:
        def result = helper.get_auth_route(criteria)

        def expect_result = "http://127.0.0.1:8000/v1/auth/sign-up"
        then:
        println result
        result == expect_result
    }
}
