#!/usr/bin/env python3

import json
import asyncio
import aiohttp
import time


async def create(id, k, dimension):
    resp = await aiohttp.ClientSession().request(
        "post", 'http://localhost:8080/index',
        data=json.dumps({"id": id, "k": k, "dimension": dimension}),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


async def insert(index_id, id, vec):
    resp = await aiohttp.ClientSession().request(
        "post", 'http://localhost:8080/index/' + str(index_id) + '/insert',
        data=json.dumps({"id_user": id,
                         "vector": vec}),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


async def search(index_id, vector):
    resp = await aiohttp.ClientSession().request(
        "post", 'http://localhost:8080/index/' + str(index_id) + '/search',
        data=json.dumps({"vector": vector}),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


async def list_indices():
    resp = await aiohttp.ClientSession().request(
        "get", 'http://localhost:8080/index')
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


async def get_index_info(id):
    resp = await aiohttp.ClientSession().request(
        "get", 'http://localhost:8080/index/' + str(id))
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


async def delete_index(id):
    resp = await aiohttp.ClientSession().request(
        "delete", 'http://localhost:8080/index/' + str(id))
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status

if __name__ == '__main__':

    asyncio.get_event_loop().run_until_complete(create(id=0, k=10, dimension=3))
    time.sleep(2)

    asyncio.get_event_loop().run_until_complete(list_indices())
    time.sleep(2)

    asyncio.get_event_loop().run_until_complete(get_index_info(0))
    time.sleep(2)

    asyncio.get_event_loop().run_until_complete(
        insert(index_id=0, id=0, vec=[2.0, 8.2, 2.3]))
    asyncio.get_event_loop().run_until_complete(
        insert(index_id=0, id=1, vec=[2.0, 8.2, 3.3]))
    asyncio.get_event_loop().run_until_complete(
        insert(index_id=0, id=2, vec=[2.0, 8.2, 4.3]))
    time.sleep(2)

    asyncio.get_event_loop().run_until_complete(get_index_info(0))
    time.sleep(2)
    asyncio.get_event_loop().run_until_complete(get_index_info(0))
    time.sleep(2)
    asyncio.get_event_loop().run_until_complete(delete_index(0))
    time.sleep(2)

    asyncio.get_event_loop().run_until_complete(get_index_info(0))
    time.sleep(2)

    # asyncio.get_event_loop().run_until_complete(insert(id=0, vec=[2.0, 8.2, 6.3]))
    # asyncio.get_event_loop().run_until_complete(insert(id=1, vec=[1.0, 8.2, 3.3]))
    # asyncio.get_event_loop().run_until_complete(insert(id=2, vec=[9.0, 2.2, 6.3]))
    # asyncio.get_event_loop().run_until_complete(insert(id=3, vec=[9.0, 2.2, 6.3]))
    # asyncio.get_event_loop().run_until_complete(insert(id=4, vec=[0.0, 0.0, 0.0]))
    # asyncio.get_event_loop().run_until_complete(insert(id=5, vec=[2.0, 8.2, 6.3]))
    # time.sleep(2)
    # asyncio.get_event_loop().run_until_complete(search())
