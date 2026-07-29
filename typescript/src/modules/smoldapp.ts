import type { IconSource } from "../source";

const ASSETS_BASE_URL = "https://raw.githubusercontent.com/smoldapp/tokenassets/main";

const MAINNET_NETWORK_ID = 1;

const MAINNET_NATIVE_URL = `${ASSETS_BASE_URL}/tokens/1/0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee/logo.svg`;

/** @see https://tokens.smold.app/ethereum */
export const smoldapp: IconSource = {
  name: "smoldapp",
  url: (query) => {
    if (query.type === "network") return `${ASSETS_BASE_URL}/chains/${query.networkId}/logo.svg`;

    if (query.type === "native") {
      return query.networkId === MAINNET_NETWORK_ID ? MAINNET_NATIVE_URL : undefined;
    }

    return `${ASSETS_BASE_URL}/tokens/${query.networkId}/${query.address.toLowerCase()}/logo.svg`;
  },
};
