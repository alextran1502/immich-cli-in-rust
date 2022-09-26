# \AssetApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_duplicate_asset**](AssetApi.md#check_duplicate_asset) | **POST** /asset/check | 
[**delete_asset**](AssetApi.md#delete_asset) | **DELETE** /asset | 
[**download_file**](AssetApi.md#download_file) | **GET** /asset/download | 
[**get_all_assets**](AssetApi.md#get_all_assets) | **GET** /asset | 
[**get_asset_by_id**](AssetApi.md#get_asset_by_id) | **GET** /asset/assetById/{assetId} | 
[**get_asset_by_time_bucket**](AssetApi.md#get_asset_by_time_bucket) | **POST** /asset/time-bucket | 
[**get_asset_count_by_time_bucket**](AssetApi.md#get_asset_count_by_time_bucket) | **POST** /asset/count-by-time-bucket | 
[**get_asset_count_by_user_id**](AssetApi.md#get_asset_count_by_user_id) | **GET** /asset/count-by-user-id | 
[**get_asset_search_terms**](AssetApi.md#get_asset_search_terms) | **GET** /asset/search-terms | 
[**get_asset_thumbnail**](AssetApi.md#get_asset_thumbnail) | **GET** /asset/thumbnail/{assetId} | 
[**get_curated_locations**](AssetApi.md#get_curated_locations) | **GET** /asset/curated-locations | 
[**get_curated_objects**](AssetApi.md#get_curated_objects) | **GET** /asset/curated-objects | 
[**get_user_assets_by_device_id**](AssetApi.md#get_user_assets_by_device_id) | **GET** /asset/{deviceId} | 
[**search_asset**](AssetApi.md#search_asset) | **POST** /asset/search | 
[**serve_file**](AssetApi.md#serve_file) | **GET** /asset/file | 
[**upload_file**](AssetApi.md#upload_file) | **POST** /asset/upload | 



## check_duplicate_asset

> crate::models::CheckDuplicateAssetResponseDto check_duplicate_asset(check_duplicate_asset_dto)


Check duplicated asset before uploading - for Web upload used

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_duplicate_asset_dto** | [**CheckDuplicateAssetDto**](CheckDuplicateAssetDto.md) |  | [required] |

### Return type

[**crate::models::CheckDuplicateAssetResponseDto**](CheckDuplicateAssetResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_asset

> Vec<crate::models::DeleteAssetResponseDto> delete_asset(delete_asset_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_asset_dto** | [**DeleteAssetDto**](DeleteAssetDto.md) |  | [required] |

### Return type

[**Vec<crate::models::DeleteAssetResponseDto>**](DeleteAssetResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file

> serde_json::Value download_file(aid, did, is_thumb, is_web)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |
**did** | **String** |  | [required] |
**is_thumb** | Option<**bool**> |  |  |
**is_web** | Option<**bool**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_assets

> Vec<crate::models::AssetResponseDto> get_all_assets()


Get all AssetEntity belong to the user

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_by_id

> crate::models::AssetResponseDto get_asset_by_id(asset_id)


Get a single asset's information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |

### Return type

[**crate::models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_by_time_bucket

> Vec<crate::models::AssetResponseDto> get_asset_by_time_bucket(get_asset_by_time_bucket_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_asset_by_time_bucket_dto** | [**GetAssetByTimeBucketDto**](GetAssetByTimeBucketDto.md) |  | [required] |

### Return type

[**Vec<crate::models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_count_by_time_bucket

> crate::models::AssetCountByTimeBucketResponseDto get_asset_count_by_time_bucket(get_asset_count_by_time_bucket_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_asset_count_by_time_bucket_dto** | [**GetAssetCountByTimeBucketDto**](GetAssetCountByTimeBucketDto.md) |  | [required] |

### Return type

[**crate::models::AssetCountByTimeBucketResponseDto**](AssetCountByTimeBucketResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_count_by_user_id

> crate::models::AssetCountByUserIdResponseDto get_asset_count_by_user_id()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AssetCountByUserIdResponseDto**](AssetCountByUserIdResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_search_terms

> Vec<String> get_asset_search_terms()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_thumbnail

> serde_json::Value get_asset_thumbnail(asset_id, format)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** |  | [required] |
**format** | Option<[**ThumbnailFormat**](.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_curated_locations

> Vec<crate::models::CuratedLocationsResponseDto> get_curated_locations()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CuratedLocationsResponseDto>**](CuratedLocationsResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_curated_objects

> Vec<crate::models::CuratedObjectsResponseDto> get_curated_objects()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CuratedObjectsResponseDto>**](CuratedObjectsResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_assets_by_device_id

> Vec<String> get_user_assets_by_device_id(device_id)


Get all asset of a device that are in the database, ID only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_asset

> Vec<crate::models::AssetResponseDto> search_asset(search_asset_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_asset_dto** | [**SearchAssetDto**](SearchAssetDto.md) |  | [required] |

### Return type

[**Vec<crate::models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serve_file

> serde_json::Value serve_file(aid, did, is_thumb, is_web)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |
**did** | **String** |  | [required] |
**is_thumb** | Option<**bool**> |  |  |
**is_web** | Option<**bool**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> crate::models::AssetFileUploadResponseDto upload_file(asset_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_data** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::AssetFileUploadResponseDto**](AssetFileUploadResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

