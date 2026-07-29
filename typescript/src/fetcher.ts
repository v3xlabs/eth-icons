import type { IconError } from "./error";
import type { IconResult } from "./result";

export type FetchFunction = typeof globalThis.fetch;

export type JsonResult
  = | { type: "ok"; body: unknown; }
    | { type: "not-found"; }
    | { type: "failed"; error: IconError; };

export type IconFetcher = {
  fetchImageUrl: (url: string) => Promise<IconResult>;
  fetchJson: (url: string) => Promise<JsonResult>;
};

export type IconFetcherOptions = {
  fetch?: FetchFunction;
  headers?: HeadersInit;
  signal?: AbortSignal;
};

type ResponseResult
  = | { type: "ok"; response: Response; }
    | { type: "not-found"; }
    | { type: "failed"; error: IconError; };

const NOT_FOUND_STATUS = 404;

export const createIconFetcher = (options: IconFetcherOptions = {}): IconFetcher => {
  const fetchFunction = options.fetch ?? globalThis.fetch;

  const get = async (url: string): Promise<ResponseResult> => {
    try {
      const response = await fetchFunction(url, {
        headers: options.headers,
        signal: options.signal,
      });

      if (response.status === NOT_FOUND_STATUS) return { type: "not-found" };

      if (!response.ok) {
        return { type: "failed", error: { type: "http", url, status: response.status } };
      }

      return { type: "ok", response };
    }
    catch (error) {
      return { type: "failed", error: { type: "network", url, cause: error } };
    }
  };

  return {
    fetchImageUrl: async (url) => {
      const result = await get(url);

      if (result.type !== "ok") return result;

      try {
        const buffer = await result.response.arrayBuffer();

        return {
          type: "found",
          icon: {
            bytes: new Uint8Array(buffer),
            mimeType: result.response.headers.get("content-type") ?? undefined,
          },
        };
      }
      catch (error) {
        return { type: "failed", error: { type: "decode", url, cause: error } };
      }
    },

    fetchJson: async (url) => {
      const result = await get(url);

      if (result.type !== "ok") return result;

      try {
        return { type: "ok", body: await result.response.json() };
      }
      catch (error) {
        return { type: "failed", error: { type: "decode", url, cause: error } };
      }
    },
  };
};
