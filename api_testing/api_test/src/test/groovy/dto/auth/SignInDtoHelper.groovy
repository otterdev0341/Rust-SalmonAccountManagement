package dto.auth

import groovy.transform.ToString

@ToString
class SignInDtoHelper {

    /*
        1. valid {email} {password}
        2. invalid {email} {password}

        3. invalid {email} valid {password}
        4. valid {email} invalid {password}

        5. valid {email} empty {password}
        6. valid {password} empty {email}

        7. empty {email} empty {password}

    * */



    ReqSignDto case_1_valid_email_valid_password() {
        return new ReqSignDto(
            email: "kotaro@work.com",
            password: "kotaro1235555"
        )
    }

    ReqSignDto case_2_invalid_email_and_password(){
        return new ReqSignDto(
            email: "abcz@gmail.com",
            password: "secreat_password78924"
        )
    }

    ReqSignDto case_3_invalid_email_valid_password(){
        return new ReqSignDto(
            email: "abcz@gmail.com",
            password: "kotaro1235555"
        )
    }

    ReqSignDto case_4_valid_email_invalid_password(){
        return new ReqSignDto(
            email: "kotaro@work.com",
            password: "secreat_password78924"
        )
    }

    ReqSignDto case_5_valid_email_empty_password(){
        return new ReqSignDto(
                email: "kotaro@work.com",
                password: ""
        )
    }

    ReqSignDto case_6_valid_password_empty_email(){
        return new ReqSignDto(
                email: "",
                password: "kotaro1235555"
        )
    }

    ReqSignDto case_7_empty_email_empty_password(){
        return new ReqSignDto(
                email: "",
                password: ""
        )
    }
}
