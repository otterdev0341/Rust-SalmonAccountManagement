package dto.company

import groovy.transform.ToString


interface ViewCompanyResponse<T> {
    String getStatus()
    T getData()
}


@ToString
class ResEntryCompanyDto {
    String id
    String name
    String description
    String createdAt
    String updatedAt
}

class SuccessViewCompanyResponse implements ViewCompanyResponse<ResEntryCompanyDto> {
    String status
    ResEntryCompanyDto data
}