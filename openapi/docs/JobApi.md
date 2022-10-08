# \JobApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_jobs_status**](JobApi.md#get_all_jobs_status) | **GET** /jobs | 
[**get_job_status**](JobApi.md#get_job_status) | **GET** /jobs/{jobId} | 
[**send_job_command**](JobApi.md#send_job_command) | **PUT** /jobs/{jobId} | 



## get_all_jobs_status

> crate::models::AllJobStatusResponseDto get_all_jobs_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AllJobStatusResponseDto**](AllJobStatusResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_status

> crate::models::JobStatusResponseDto get_job_status(job_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | [**JobId**](.md) |  | [required] |

### Return type

[**crate::models::JobStatusResponseDto**](JobStatusResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_job_command

> f32 send_job_command(job_id, job_command_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | [**JobId**](.md) |  | [required] |
**job_command_dto** | [**JobCommandDto**](JobCommandDto.md) |  | [required] |

### Return type

**f32**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

