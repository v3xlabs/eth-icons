import type { IconSource } from "../source";

export const avara: IconSource = {
  name: "avara",
  url: query =>
    (query.type === "erc20"
      ? `https://token-logos.family.co/asset?id=${query.networkId}:${query.address.toLowerCase()}`
      : undefined),
};
