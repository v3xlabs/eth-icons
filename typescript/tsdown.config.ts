import { defineConfig } from "tsdown";

// eslint-disable-next-line import/no-default-export
export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
});
