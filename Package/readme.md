## Installation

```
npm i @hedystia/strange

pnpm i @hedystia/strange

yarn add @hedystia/strange
```

## Nodejs Version

- `v18.0.0` or higher

## Links

- [Discord](https://discord.gg/aXvuUpvRQs) [Hedystia Discord]
- [Discord_Bot](https://hedystia.com) [Hedystia Bot]
- [Strange_Support](https://discord.gg/Pt97u5MkAr) [Strange Support]
- [Strange_Docs](https://strangeapi.hostz.me/docs) [Strange Docs]
- [Strange_Api_Key](https://strangeapi.hostz.me/dasbboard) [Strange Dashboard]
- [Strange_Api_Key](https://discord.gg/Pt97u5MkAr) [Strange Support]
- [Npm_Docs](https://docs.hedystia.com/strange/start) [Strange.Api docs]

## Example

```js
const Strange = require("@hedystia/strange");
const {Buffer} = require("buffer");

// or

import Strange from "@hedystia/strange";
import {Buffer} from "buffer";

process.loadEnvFile("./.env");

const api = new Strange(process.env.API_KEY);

const buf = api.challenger({
  image: process.env.IMAGE_URL,
  silhouetted: true,
});

console.log(Buffer.from(buf));
```
