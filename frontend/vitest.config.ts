import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // Testlerin cogu saf mantik; jsdom yalnizca window/localStorage'a dokunan
    // birkac yerde lazim ve tumune vermek daha basit.
    environment: "jsdom",
    include: ["src/**/*.test.{ts,tsx}"],
    coverage: {
      provider: "v8",
      include: ["src/lib/**", "src/stores/downloadsReducer.ts", "src/i18n/**"],
    },
  },
});
