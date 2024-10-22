wrk.method = "POST"
wrk.body = '{"operationName": "BigQuery","variables": {"delay": 0,"bigObjects": 3,"deeplyNestedObjects": 3,"nestedObjects": 3},"query": "query BigQuery($delay: Int!, $bigObjects: Int!, $deeplyNestedObjects: Int!, $nestedObjects: Int!) { bigResponse( artificialDelay: $delay bigObjects: $bigObjects deeplyNestedObjects: $deeplyNestedObjects nestedObjects: $nestedObjects ) { nestedObjects { ...DeeplyNestedFields } } } fragment DeeplyNestedFields on NestedObject { deeplyNestedObjects { aFieldOnDeeplyNestedObject bFieldOnDeeplyNestedObject cFieldOnDeeplyNestedObject dFieldOnDeeplyNestedObject eFieldOnDeeplyNestedObject fFieldOnDeeplyNestedObject gFieldOnDeeplyNestedObject hFieldOnDeeplyNestedObject iFieldOnDeeplyNestedObject jFieldOnDeeplyNestedObject kFieldOnDeeplyNestedObject lFieldOnDeeplyNestedObject mFieldOnDeeplyNestedObject nFieldOnDeeplyNestedObject oFieldOnDeeplyNestedObject pFieldOnDeeplyNestedObject qFieldOnDeeplyNestedObject rFieldOnDeeplyNestedObject sFieldOnDeeplyNestedObject tFieldOnDeeplyNestedObject uFieldOnDeeplyNestedObject vFieldOnDeeplyNestedObject wFieldOnDeeplyNestedObject xFieldOnDeeplyNestedObject yFieldOnDeeplyNestedObject zFieldOnDeeplyNestedObject } }"}'
wrk.headers["Connection"] = "keep-alive"
wrk.headers["User-Agent"] =
	"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
wrk.headers["Content-Type"] = "application/json"