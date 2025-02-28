package dto.auth

import groovy.transform.ToString


@ToString
class ResEntryUserDto {
    String id
    String username
    String firstName
    String lastName
    String email
    String createdAt
    String updatedAt
}
