import type { IconSource } from "../source";

const ASSETS_BASE_URL = "https://safe-transaction-assets.safe.global/chains";

export const safewallet: IconSource = {
  name: "safewallet",
  url: (query) => {
    if (query.type === "network") return `${ASSETS_BASE_URL}/${query.networkId}/chain_logo.png`;

    if (query.type === "native") {
      return `${ASSETS_BASE_URL}/${query.networkId}/currency_logo.png`;
    }

    return undefined;
  },
};
