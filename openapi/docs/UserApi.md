# \UserApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_profile_image**](UserApi.md#create_profile_image) | **POST** /user/profile-image | 
[**create_user**](UserApi.md#create_user) | **POST** /user | 
[**get_all_users**](UserApi.md#get_all_users) | **GET** /user | 
[**get_my_user_info**](UserApi.md#get_my_user_info) | **GET** /user/me | 
[**get_profile_image**](UserApi.md#get_profile_image) | **GET** /user/profile-image/{userId} | 
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /user/info/{userId} | 
[**get_user_count**](UserApi.md#get_user_count) | **GET** /user/count | 
[**update_user**](UserApi.md#update_user) | **PUT** /user | 



## create_profile_image

> crate::models::CreateProfileImageResponseDto create_profile_image(file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::CreateProfileImageResponseDto**](CreateProfileImageResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::UserResponseDto create_user(create_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_dto** | [**CreateUserDto**](CreateUserDto.md) |  | [required] |

### Return type

[**crate::models::UserResponseDto**](UserResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_users

> Vec<crate::models::UserResponseDto> get_all_users(is_all)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_all** | **bool** |  | [required] |

### Return type

[**Vec<crate::models::UserResponseDto>**](UserResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_user_info

> crate::models::UserResponseDto get_my_user_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserResponseDto**](UserResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_image

> serde_json::Value get_profile_image(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> crate::models::UserResponseDto get_user_by_id(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::UserResponseDto**](UserResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_count

> crate::models::UserCountResponseDto get_user_count()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserCountResponseDto**](UserCountResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserResponseDto update_user(update_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_dto** | [**UpdateUserDto**](UpdateUserDto.md) |  | [required] |

### Return type

[**crate::models::UserResponseDto**](UserResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

