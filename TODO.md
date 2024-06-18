
## TODO: 

- API redesign

- include Tiny LFU cache (moka)

- Optimize key design and creation

- Benchmark against Faiss

- Improve efficiency

- Include logging (hierarchical lgging: Test speed carefully!)

- Extra test coverage doesn't hurt ;)

- Don't allow index IDs to be the same... check with hashed ID

- Remove element from index

- Reverse sorted list optimization in index

- Dependencies libstdc++6 (>= 6), libc6 (>= 2.33)


## DONE

- API: design in swagger.yml then testing with actix_rt

- Include key value store (Rocksdb)

- Implement key design

- get userIDs instead of hashes more efficiently when searching!