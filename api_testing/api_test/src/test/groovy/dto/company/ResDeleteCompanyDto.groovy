package dto.company

import groovy.transform.ToString

interface DeleteCompanyResponse<T>{
    String getStatus()
    T getData()
}

@ToString
class ResDeleteCompanyDto {
    String status
    String data
}




