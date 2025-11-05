import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  target: "esnext",
  format: ["esm"],
  dts: {
    resolve: true,
  },
  sourcemap: true,
  clean: true,
  minify: true,
  terserOptions: {
    // Ensure the options are compatible with the specified terser version
    format: {
      comments: false,
    },
    compress: {
      drop_console: true,
    },
  },
});
