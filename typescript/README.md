<h1 align="center">
    eth-icons
</h1>

<p align="center">
  A smol typescript library for fetching Ethereum asset icons from various sources.
</p>
<p align="center">
    <a href="https://www.npmjs.com/package/eth-icons"><img src="https://img.shields.io/badge/npm-eth--icons-red?logo=npm&style=flat" alt="npm"></a>
    <a href="https://github.com/v3xlabs/eth-icons"><img src="https://img.shields.io/badge/Repository-v3xlabs/eth--icons-blue?style=flat" alt="Repository"></a>
    <a href="#"><img src="https://img.shields.io/badge/License-LGPL--3.0-hotpink?style=flat" alt="License: LGPL-3.0"></a>
</p>

> [!IMPORTANT]
> eth-icons uses third party services to fetch iconography, not ethereum rpc calls.

Network (Mainnet, Sepolia, etc.), Native Asset (ETH, sepETH, etc.), and ERC20 (wETH, etc.) icons are an inherit non-standardized extension of the ethereum protocol.
To allow for easily integrating these into your application eth-icons aims to bring a set of helpers to obtain iconography from various sources.

## Quickstart

```sh
pnpm add eth-icons
```

```ts
import { createIconClient } from "eth-icons";

const icons = createIconClient();

const results = await icons.resolve({
  type: "erc20",
  networkId: 1,
  address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
});
```

`resolve` never rejects. Each source reports one of four outcomes, and a source that fails does not
affect the others:

```ts
for (const { source, result } of results) {
  switch (result.type) {
    case "found": console.log(source, result.icon.mimeType); break;
    case "not-found": break;
    case "unsupported": break;
    case "failed": console.warn(source, result.error); break;
  }
}
```

## Data Sources

Currently supported data sources include:

- [Avara](./src/modules/avara.ts) - supports ERC20s
- [Blockscout](./src/modules/blockscout.ts) - supports ERC20s
- [SafeWallet](./src/modules/safewallet.ts) - supports network and native token icons
- [Smoldapp](./src/modules/smoldapp.ts) - supports network, native tokens, and ERC20s
- [Zerion](./src/modules/zerion.ts) - supports ERC20s

## Selecting Sources

Every source is opt-in. `createIconClient()` registers all of them, and `defaultSources` lets you
extend that set instead of replacing it.

```ts
import { blockscout, createIconClient, defaultSources, zerion } from "eth-icons";

const icons = createIconClient({
  sources: [blockscout, zerion],
});

const withCustom = createIconClient({
  sources: [...defaultSources, mySource],
});
```

## Bring your own fetch

```ts
const icons = createIconClient({
  fetch: myFetch,
  headers: { "user-agent": "my-app" },
  signal: controller.signal,
});
```

A per-call `signal` overrides the client one, which makes cancel-on-unmount a one-liner:

```ts
const results = await icons.resolve(query, { signal: controller.signal });
```

## Using an icon

An icon carries its raw bytes and the mime type the source served it with.

```ts
import { toBlob, toDataUri } from "eth-icons";

URL.createObjectURL(toBlob(icon));
element.src = toDataUri(icon);
```
