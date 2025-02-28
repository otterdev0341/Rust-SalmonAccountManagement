package dto.auth

import utility.HelpRandomNum

class SignUpDtoHelper {

    ReqCreateUserDto get_always_new_user() {

        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()

        return new ReqCreateUserDto(
                firstName: "test_user${random_num}",
                lastName: "test_user${random_num}",
                email: "test_${random_num}@gmail.com",
                username: "test_${random_num}",
                password: "test_${random_num}"
        )
    }

    ReqCreateUserDto get_empty_first_name_field() {
        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()
        return new ReqCreateUserDto(
                firstName: "",
                lastName: "test_user${random_num}",
                email: "test_${random_num}@gmail.com",
                username: "test_${random_num}",
                password: "test_${random_num}"
        )
    }

    ReqCreateUserDto get_empty_last_name_field() {
        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()
        return new ReqCreateUserDto(
                firstName: "test_user${random_num}",
                lastName: "",
                email: "test_${random_num}@gmail.com",
                username: "test_${random_num}",
                password: "test_${random_num}"
        )
    }

    ReqCreateUserDto get_empty_email_field() {
        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()
        return new ReqCreateUserDto(
                firstName: "test_user${random_num}",
                lastName: "test_user${random_num}",
                email: "",
                username: "test_${random_num}",
                password: "test_${random_num}"
        )
    }

    ReqCreateUserDto get_empty_password_field() {
        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()
        return new ReqCreateUserDto(
                firstName: "test_user${random_num}",
                lastName: "test_user${random_num}",
                email: "test_${random_num}@gmail.com",
                username: "test_${random_num}",
                password: ""
        )
    }

    ReqCreateUserDto get_exist_user_data(){
        return new ReqCreateUserDto(
                firstName: "kotaro",
                lastName: "cute",
                password: "kotaro1235555",
                email: "kotaro@work.com",
                username: "kotaro_cute"
        )
    }

    ReqCreateUserDto get_exist_user_email() {
        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()

        return new ReqCreateUserDto(
                firstName: "test_user${random_num}",
                lastName: "test_user${random_num}",
                email: "kotaro@gmail.com",
                username: "test_${random_num}",
                password: "abcdefh1123"
        )
    }

    ReqCreateUserDto get_exist_username_field() {
        def helper = new HelpRandomNum()
        def random_num = helper.get_random_num()

        return new ReqCreateUserDto(
                firstName: "test_user${random_num}",
                lastName: "test_user${random_num}",
                email: "test_${random_num}@gmail.com",
                username: "kotaro_cute",
                password: "abcdef1123"
        )
    }


}
