import type { IconError } from "./error";
import type { Icon } from "./icon";

export type IconResult
  = | { type: "found"; icon: Icon; }
    | { type: "not-found"; }
    | { type: "unsupported"; }
    | { type: "failed"; error: IconError; };

export type SourceResult = {
  source: string;
  result: IconResult;
};

export type SourceError = {
  source: string;
  error: IconError;
};

type FailedSourceResult = SourceResult & { result: Extract<IconResult, { type: "failed"; }>; };

const isFailed = (entry: SourceResult): entry is FailedSourceResult =>
  entry.result.type === "failed";

export const findErrors = (results: readonly SourceResult[]): SourceError[] =>
  results.filter(isFailed).map(({ source, result }) => ({ source, error: result.error }));
