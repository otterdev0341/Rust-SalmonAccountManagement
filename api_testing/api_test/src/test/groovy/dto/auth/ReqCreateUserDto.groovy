package dto.auth

import groovy.transform.ToString

@ToString
class ReqCreateUserDto {
    String username
    String firstName
    String lastName
    String email
    String password

}
