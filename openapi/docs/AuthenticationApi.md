# \AuthenticationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_sign_up**](AuthenticationApi.md#admin_sign_up) | **POST** /auth/admin-sign-up | 
[**login**](AuthenticationApi.md#login) | **POST** /auth/login | 
[**logout**](AuthenticationApi.md#logout) | **POST** /auth/logout | 
[**validate_access_token**](AuthenticationApi.md#validate_access_token) | **POST** /auth/validateToken | 



## admin_sign_up

> crate::models::AdminSignupResponseDto admin_sign_up(sign_up_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_dto** | [**SignUpDto**](SignUpDto.md) |  | [required] |

### Return type

[**crate::models::AdminSignupResponseDto**](AdminSignupResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> crate::models::LoginResponseDto login(login_credential_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_credential_dto** | [**LoginCredentialDto**](LoginCredentialDto.md) |  | [required] |

### Return type

[**crate::models::LoginResponseDto**](LoginResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> crate::models::LogoutResponseDto logout()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LogoutResponseDto**](LogoutResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_access_token

> crate::models::ValidateAccessTokenResponseDto validate_access_token()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ValidateAccessTokenResponseDto**](ValidateAccessTokenResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

