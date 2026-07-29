import { IconSource } from "../client";

export const safewallet: IconSource = {
    url: query => {
        if (query.type == 'network-icon') return `https://safe-transaction-assets.safe.global/chains/${query.networkId}/chain_logo.png`;

        if (query.type == 'native-icon') return `https://safe-transaction-assets.safe.global/chains/${query.networkId}/currency_logo.png`;
    }
};
