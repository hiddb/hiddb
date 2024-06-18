#!/bin/bash

collectionId=test
fieldId=test

 curl -L -X POST 'localhost:8080/collection' \
 -H 'Content-Type: application/json' \
 -H 'Accept: application/json' \
 --data-raw '{
   "collection_id": "'${collectionid}'"
 }'
 
 curl -L -X POST "localhost:8080/collection/${collectionId}/index" \
 -H 'Content-Type: application/json' \
 --data-raw '{
   "field_id": "'${fieldId}'",
   "dimension": 3
 }'


curl 	-d '{"documents":[{"'$fieldId'":[1.0, 2.0, 3.0], "id":"string"}]}' \
  -H "Content-Type: application/json" \
  -X POST "localhost:8080/collection/${collectionId}/document"


curl 	-d '{"field_id":"'$fieldId'", "vector": [1.0, 2.0, 3.0]}' \
  -H "Content-Type: application/json" \
  -X POST "localhost:8080/collection/${collectionId}/document/search"
