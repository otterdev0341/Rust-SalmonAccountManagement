package utility

import io.qameta.allure.Epic
import spock.lang.Specification
import java.util.*;

class HelpRandomNum {
    String get_random_num() {
        def random = new Random().nextInt(9000) + 10000
        return random.toString()
    }
}


@Epic("Test internal function")
class TestHelpRandomNum extends Specification {

    def "test_function"() {
        given: "an instance of HelpRandomNum"
        HelpRandomNum helper = new HelpRandomNum()

        when: "calling get_random_num"
        String randomNumber = helper.get_random_num()

        then: "the result is printed"
        println randomNumber
    }
}