# \AlbumApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_assets_to_album**](AlbumApi.md#add_assets_to_album) | **PUT** /album/{albumId}/assets | 
[**add_users_to_album**](AlbumApi.md#add_users_to_album) | **PUT** /album/{albumId}/users | 
[**create_album**](AlbumApi.md#create_album) | **POST** /album | 
[**delete_album**](AlbumApi.md#delete_album) | **DELETE** /album/{albumId} | 
[**get_album_count_by_user_id**](AlbumApi.md#get_album_count_by_user_id) | **GET** /album/count-by-user-id | 
[**get_album_info**](AlbumApi.md#get_album_info) | **GET** /album/{albumId} | 
[**get_all_albums**](AlbumApi.md#get_all_albums) | **GET** /album | 
[**remove_asset_from_album**](AlbumApi.md#remove_asset_from_album) | **DELETE** /album/{albumId}/assets | 
[**remove_user_from_album**](AlbumApi.md#remove_user_from_album) | **DELETE** /album/{albumId}/user/{userId} | 
[**update_album_info**](AlbumApi.md#update_album_info) | **PATCH** /album/{albumId} | 



## add_assets_to_album

> crate::models::AlbumResponseDto add_assets_to_album(album_id, add_assets_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |
**add_assets_dto** | [**AddAssetsDto**](AddAssetsDto.md) |  | [required] |

### Return type

[**crate::models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_users_to_album

> crate::models::AlbumResponseDto add_users_to_album(album_id, add_users_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |
**add_users_dto** | [**AddUsersDto**](AddUsersDto.md) |  | [required] |

### Return type

[**crate::models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_album

> crate::models::AlbumResponseDto create_album(create_album_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_album_dto** | [**CreateAlbumDto**](CreateAlbumDto.md) |  | [required] |

### Return type

[**crate::models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_album

> delete_album(album_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_count_by_user_id

> crate::models::AlbumCountResponseDto get_album_count_by_user_id()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlbumCountResponseDto**](AlbumCountResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_info

> crate::models::AlbumResponseDto get_album_info(album_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |

### Return type

[**crate::models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_albums

> Vec<crate::models::AlbumResponseDto> get_all_albums(shared, asset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shared** | Option<**bool**> |  |  |
**asset_id** | Option<**String**> | Only returns albums that contain the asset Ignores the shared parameter undefined: get all albums |  |

### Return type

[**Vec<crate::models::AlbumResponseDto>**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_asset_from_album

> crate::models::AlbumResponseDto remove_asset_from_album(album_id, remove_assets_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |
**remove_assets_dto** | [**RemoveAssetsDto**](RemoveAssetsDto.md) |  | [required] |

### Return type

[**crate::models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_album

> remove_user_from_album(album_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_album_info

> crate::models::AlbumResponseDto update_album_info(album_id, update_album_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **String** |  | [required] |
**update_album_dto** | [**UpdateAlbumDto**](UpdateAlbumDto.md) |  | [required] |

### Return type

[**crate::models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

