import type { IconFetcher } from "./fetcher";
import type { IconResult } from "./result";

export type NetworkId = number;

export type Address = `0x${string}`;

const ADDRESS_PATTERN = /^0x[\da-f]{40}$/i;

export const isAddress = (value: string): value is Address => ADDRESS_PATTERN.test(value);

export type IconQuery
  = | { type: "network"; networkId: NetworkId; }
    | { type: "native"; networkId: NetworkId; }
    | { type: "erc20"; networkId: NetworkId; address: Address; };

export type IconSource = {
  name: string;
  url: (query: IconQuery) => string | undefined;
  fetch?: (fetcher: IconFetcher, query: IconQuery) => Promise<IconResult>;
};
