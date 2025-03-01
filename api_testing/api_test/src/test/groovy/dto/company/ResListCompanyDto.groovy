package dto.company

import groovy.transform.ToString

interface ViewCompaniesResponse<T>{
    String getStatus()
    T getData()
}

@ToString
class ResListCompanyDto {
    int total
    List<ResEntryCompanyDto> companies
}

class SuccessViewCompaniesResponse implements ViewCompaniesResponse<ResListCompanyDto> {
    String status
    ResListCompanyDto data
}
