import type { IconSource } from "../source";

export const zerion: IconSource = {
  name: "zerion",
  url: query =>
    (query.type === "erc20"
      ? `https://cdn.zerion.io/${query.address.toLowerCase()}.png`
      : undefined),
};
