import type { FetchFunction, IconFetcher } from "./fetcher";
import { createIconFetcher } from "./fetcher";
import { defaultSources } from "./modules/defaults";
import type { IconResult, SourceResult } from "./result";
import type { IconQuery, IconSource } from "./source";

export type IconClientOptions = {
  sources?: readonly IconSource[];
  fetch?: FetchFunction;
  headers?: HeadersInit;
  signal?: AbortSignal;
};

export type ResolveOptions = {
  signal?: AbortSignal;
};

export type IconClient = {
  sources: readonly IconSource[];
  resolve: (query: IconQuery, options?: ResolveOptions) => Promise<SourceResult[]>;
};

const fetchFromSource = async (
  source: IconSource,
  fetcher: IconFetcher,
  query: IconQuery,
): Promise<IconResult> => {
  if (source.fetch !== undefined) return source.fetch(fetcher, query);

  const url = source.url(query);

  return url === undefined ? { type: "unsupported" } : fetcher.fetchImageUrl(url);
};

export const createIconClient = (options: IconClientOptions = {}): IconClient => {
  const sources = options.sources ?? defaultSources;

  return {
    sources,
    resolve: async (query, resolveOptions = {}) => {
      const fetcher = createIconFetcher({
        fetch: options.fetch,
        headers: options.headers,
        signal: resolveOptions.signal ?? options.signal,
      });

      return Promise.all(
        sources.map(async source => ({
          source: source.name,
          result: await fetchFromSource(source, fetcher, query),
        })),
      );
    },
  };
};
