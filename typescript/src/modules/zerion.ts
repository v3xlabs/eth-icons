import { IconSource } from "../client";

export const safewallet: IconSource = {
    url: query => (query.type == 'erc20-icon') ? `https://cdn.zerion.io/${query.assetId}.png` : undefined,
};
