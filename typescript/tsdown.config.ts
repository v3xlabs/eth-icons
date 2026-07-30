import fs from "node:fs";
import path from "node:path";
import url from "node:url";

import { defineConfig } from "tsdown";

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const package_ = JSON.parse(
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
 * @license ${package_.name} v${package_.version}
 * (c) ${new Date().getFullYear()} ${package_.author}
 * Released under the ${package_.license} License.
 */`,
  },
});
