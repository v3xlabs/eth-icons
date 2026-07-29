import type { IconQuery, IconSource } from "../source";

const MAINNET_NETWORK_ID = 1;

const metadataUrl = (query: IconQuery): string | undefined =>
  (query.type === "erc20" && query.networkId === MAINNET_NETWORK_ID
    ? `https://eth.blockscout.com/api/v2/tokens/${query.address.toLowerCase()}`
    : undefined);

// The token payload mixes nulls and non-string values, so icon_url has to be narrowed, not trusted.
const readIconUrl = (body: unknown): string | undefined => {
  if (typeof body !== "object" || body === null || !("icon_url" in body)) return undefined;

  const iconUrl = body.icon_url;

  return typeof iconUrl === "string" ? iconUrl : undefined;
};

export const blockscout: IconSource = {
  name: "blockscout",
  url: metadataUrl,
  fetch: async (fetcher, query) => {
    const url = metadataUrl(query);

    if (url === undefined) return { type: "unsupported" };

    const metadata = await fetcher.fetchJson(url);

    if (metadata.type !== "ok") return metadata;

    const iconUrl = readIconUrl(metadata.body);

    return iconUrl === undefined
      ? { type: "not-found" }
      : fetcher.fetchImageUrl(iconUrl);
  },
};
