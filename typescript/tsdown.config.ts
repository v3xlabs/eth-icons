import { defineConfig } from "tsdown";
import fs from "node:fs";
import path from "node:path";

const pkg = JSON.parse(
  fs.readFileSync(path.resolve(__dirname, "package.json"), "utf8"),
);

// eslint-disable-next-line import/no-default-export
export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
  sourcemap: true,
  banner: {
    js: `/**
 * @license ${pkg.name} v${pkg.version}
 * (c) ${new Date().getFullYear()} ${pkg.author}
 * Released under the ${pkg.license} License.
 */`,
  },
});
