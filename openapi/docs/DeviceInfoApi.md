# \DeviceInfoApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_device_info**](DeviceInfoApi.md#create_device_info) | **POST** /device-info | 
[**update_device_info**](DeviceInfoApi.md#update_device_info) | **PATCH** /device-info | 



## create_device_info

> crate::models::DeviceInfoResponseDto create_device_info(create_device_info_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_device_info_dto** | [**CreateDeviceInfoDto**](CreateDeviceInfoDto.md) |  | [required] |

### Return type

[**crate::models::DeviceInfoResponseDto**](DeviceInfoResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device_info

> crate::models::DeviceInfoResponseDto update_device_info(update_device_info_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_device_info_dto** | [**UpdateDeviceInfoDto**](UpdateDeviceInfoDto.md) |  | [required] |

### Return type

[**crate::models::DeviceInfoResponseDto**](DeviceInfoResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

