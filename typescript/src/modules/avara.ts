import { IconSource } from "../client";

export const avara: IconSource = {
    url: query => query.type == 'erc20-icon' ? `https://token-logos.family.co/asset?id=${query.networkId}:${query.assetId}` : undefined
};
