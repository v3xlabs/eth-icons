export type IconQuery = {
    type: 'network-icon',
    networkId: number;
} | {
    type: 'native-icon',
    networkId: number;
} | {
    type: 'erc20-icon',
    networkId: number;
    assetId: string;
};

export type IconResult = {
    icon: BufferSource;
};

export type IconSource = {
    url: (query: IconQuery) => String | undefined;
    fetch: (fetcher: typeof fetch) => IconResult
};

// export type IconClient = {
//     fetch: () => ;
// };
