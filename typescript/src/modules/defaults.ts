import type { IconSource } from "../source";
import { avara } from "./avara";
import { blockscout } from "./blockscout";
import { safewallet } from "./safewallet";
import { smoldapp } from "./smoldapp";
import { zerion } from "./zerion";

export const defaultSources: readonly IconSource[] = [
  blockscout,
  avara,
  zerion,
  smoldapp,
  safewallet,
];
