package dto.company

import groovy.transform.ToString

interface ResUpdateCompanyDtoResponse<T>{
    String getStatus()
    T getData()
}

@ToString
class ResUpdateCompanyDto {
    String id
    String name
    String description
    String updatedAt
}

class SuccessEditCompanyResponse implements ResUpdateCompanyDtoResponse<ResUpdateCompanyDto> {
    String status
    ResUpdateCompanyDto data
}
