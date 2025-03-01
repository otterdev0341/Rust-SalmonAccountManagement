package route_test.auth.company.valid

import dto.company.DeleteCompanyResponse
import dto.company.ReqUpdateCompanyDto
import dto.company.ResEntryCompanyDto
import dto.company.ResListCompanyDto
import dto.company.ResUpdateCompanyDto

import dto.company.SuccessEditCompanyResponse
import dto.company.SuccessViewCompaniesResponse
import dto.company.SuccessViewCompanyResponse
import dto.company.TestCompanyDtoHelper
import groovy.contracts.Requires
import groovy.json.JsonSlurper
import io.qameta.allure.Step

import spock.lang.Shared
import spock.lang.Specification
import utility.SignInToken

class SequenceCompanyTest extends Specification {

    // create -> {save_created_id}
    // view -> use {save_created_id} to view
    // views -> check is {save_created_id} is exist
    // update -> the {save_crated_id}
    // delete -> by the id that kept
    // view -> the id that kept
    //views -> compare to latest view
    @Shared
    def create_new_user_status = false

    @Shared
    def set_view_companies = 0

    @Shared
    def is_view_companies = false

    @Shared
    def is_edit_company_done = false

    @Shared
    def company_service

    @Shared
    String user_token

    @Shared
    String company_id

    @Step("initial value for test")
    def setupSpec(){
        // inject company
        company_service = new CompanyService()
        // set user token
        user_token = new SignInToken().get_valid_token()
    }

    @Step("Test Create new company")
    def "create new company to get company"(){
        given:
        def new_company = new TestCompanyDtoHelper()._always_new_company_dto

        def response = new CompanyService().create_company(new_company, user_token)
        def inner = response.get()

        def json_response = new JsonSlurper().parseText(inner.asString())
        def response_data = json_response.data
        def created_id = response_data.idCreated

        company_id = created_id
        create_new_user_status = true
        expect:
        inner.statusCode() == 201
    }


    @Step("view the company that just created")
    @Requires({create_new_user_status})
    def "Test view company that created"(){
        given:
        def result = new CompanyService().view_company(user_token, company_id)
        def inner = result.get()
        def parsed = new JsonSlurper().parseText(inner.body().asString())

        SuccessViewCompanyResponse response = new SuccessViewCompanyResponse(
                status: parsed.status,
                data: new ResEntryCompanyDto(
                        id: parsed.data.id,
                        name: parsed.data.name,
                        description: parsed.data.description,
                        createdAt: parsed.data.createdAt,
                        updatedAt: parsed.data.updatedAt
                )
        )

        expect:
            response.data.id == company_id
            inner.getStatusCode() == 200
            inner.getStatusCode() != 201
            inner.getStatusCode() != 400
            inner.getStatusCode() != 404

    }

    @Step("view companies")
    @Requires({create_new_user_status})
    def "view companies "(){
        given:
        def result = new CompanyService().view_companies(user_token)
        def inner = result.get()
        def parsed = new JsonSlurper().parseText(inner.body().asString())

        SuccessViewCompaniesResponse response = new SuccessViewCompaniesResponse(
                status: parsed.status,
                data: new ResListCompanyDto(
                        total: parsed.data.total,
                        companies: parsed.data.companies
                )
        )
        def found_id = response.data.companies.find(it -> it.id == company_id)
        set_view_companies = response.data.total
        is_view_companies = true

        expect:
            response.data.total > 0
            found_id.id == company_id
            inner.statusCode() == 200
            inner.statusCode() != 400
            inner.statusCode() != 401
            inner.statusCode() != 404
    }

    @Step("update company detail")
    @Requires({is_view_companies })
    def "update company detail"() {
        given:
        ReqUpdateCompanyDto update_company = new TestCompanyDtoHelper().get_always_new_req_update_company_dto()
            def result = new CompanyService().edit_company(user_token, company_id, update_company)
            def inner = result.get()
            def parsed = new JsonSlurper().parseText(inner.body().asString())

        SuccessEditCompanyResponse response = new SuccessEditCompanyResponse(
                    status: parsed.status,
                    data: new ResUpdateCompanyDto(
                            id: parsed.data.id,
                            name: parsed.data.name,
                            description: parsed.data.description,
                            updatedAt: parsed.data.updatedAt
                    )
            )
        is_edit_company_done = true
        expect:
            response.data.id == company_id
            response.data.name == update_company.name
            response.data.description == update_company.description
            inner.statusCode() == 200
            inner.statusCode() != 201
            inner.statusCode() != 400
            inner.statusCode() != 404
    }





    @Step("delete company")
    @Requires({is_edit_company_done && (company_id != null)})
    def "delete the company"(){
        given:
            def result = new CompanyService().delete_company(user_token, company_id)
            def inner = result.get()
        def parsed = new JsonSlurper().parseText(inner.body().asString())

        def status = parsed.status.toString()

        expect:
            status == "success"
            inner.statusCode() == 200
            inner.statusCode() != 500
            inner.statusCode() != 400
            inner.statusCode() != 404
            inner.statusCode() != 201
    }
}