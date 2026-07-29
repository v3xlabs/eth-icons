import { IconSource } from "../client";

export const safewallet: IconSource = {
    url: query => {
        if (query.type == 'network-icon') return `https://raw.githubusercontent.com/smoldapp/tokenassets/main/chains/${query.networkId}/logo.svg`;

        if (query.type == 'native-icon' && query.networkId == 1) return "https://raw.githubusercontent.com/smoldapp/tokenassets/main/tokens/1/0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee/logo.svg";

        if (query.type == 'erc20-icon') return `https://raw.githubusercontent.com/smoldapp/tokenassets/main/tokens/${query.networkId}/${query.assetId}/logo.svg`;
    }
};
