#!/usr/bin/env python3

import json
import asyncio
import aiohttp

async def req():
    resp = await aiohttp.ClientSession().request(
        "post", 'http://localhost:8086/device',
        data=json.dumps({
            "types": "level", 
            "sdate": "2022-12-01 00:00:00", 
            "edate": "2022-12-02 23:59:59"
        }),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


asyncio.get_event_loop().run_until_complete(req())
