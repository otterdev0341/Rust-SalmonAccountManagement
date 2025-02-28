package utility

import io.qameta.allure.Epic
import spock.lang.Specification


class TestRouteManagement {
    static String base = "http://127.0.0.1:8000"

    String get_route(RouteCase routeCase) {

        switch (routeCase) {
            case RouteCase.SIGN_UP:
                return base + "/v1/auth/sign-up"
            case RouteCase.SIGN_IN:
                return base + "/v1/auth/sign-in"
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
        def criteria = RouteCase.SIGN_UP

        when:
        def result = helper.get_route(criteria)

        def expect_result = "http://127.0.0.1:8000/v1/auth/sign-up"
        then:
        println result
        result == expect_result
    }
}
