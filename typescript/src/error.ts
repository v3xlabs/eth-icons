export type IconError
  = | { type: "http"; url: string; status: number; }
    | { type: "network"; url: string; cause: unknown; }
    | { type: "decode"; url: string; cause: unknown; };
