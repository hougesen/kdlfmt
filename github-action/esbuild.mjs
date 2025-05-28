import { build } from "esbuild";

build({
  entryPoints: ["src/index.mjs"],
  bundle: true,
  minify: true,
  outdir: "dist",
  platform: "node",
  treeShaking: true,
}).catch(() => process.exit(1));
