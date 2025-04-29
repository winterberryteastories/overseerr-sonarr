# \QueueApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_queue**](QueueApi.md#delete_queue) | **DELETE** /api/v3/queue/{id} | 
[**delete_queue_bulk**](QueueApi.md#delete_queue_bulk) | **DELETE** /api/v3/queue/bulk | 
[**get_queue**](QueueApi.md#get_queue) | **GET** /api/v3/queue | 



## delete_queue

> delete_queue(id, remove_from_client, blocklist, skip_redownload, change_category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**remove_from_client** | Option<**bool**> |  |  |[default to true]
**blocklist** | Option<**bool**> |  |  |[default to false]
**skip_redownload** | Option<**bool**> |  |  |[default to false]
**change_category** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queue_bulk

> delete_queue_bulk(remove_from_client, blocklist, skip_redownload, change_category, queue_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_from_client** | Option<**bool**> |  |  |[default to true]
**blocklist** | Option<**bool**> |  |  |[default to false]
**skip_redownload** | Option<**bool**> |  |  |[default to false]
**change_category** | Option<**bool**> |  |  |[default to false]
**queue_bulk_resource** | Option<[**QueueBulkResource**](QueueBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue

> models::QueueResourcePagingResource get_queue(page, page_size, sort_key, sort_direction, include_unknown_series_items, include_series, include_episode, series_ids, protocol, languages, quality, status)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**include_unknown_series_items** | Option<**bool**> |  |  |[default to false]
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode** | Option<**bool**> |  |  |[default to false]
**series_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**protocol** | Option<[**DownloadProtocol**](.md)> |  |  |
**languages** | Option<[**Vec<i32>**](i32.md)> |  |  |
**quality** | Option<[**Vec<i32>**](i32.md)> |  |  |
**status** | Option<[**Vec<models::QueueStatus>**](models::QueueStatus.md)> |  |  |

### Return type

[**models::QueueResourcePagingResource**](QueueResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

