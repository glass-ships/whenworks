import { FlatCompat } from "@eslint/eslintrc";
import js from "@eslint/js";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
import tseslint from "typescript-eslint";
import url from "url";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: js.configs.recommended,
  // allConfig: js.configs.all,
});

export default [
  ...compat.extends(
    // "next/core-web-vitals",
    // "next/typescript",
    "prettier"
  ),
  ...tseslint.configs.recommended,
  eslintPluginPrettierRecommended,
  {
    languageOptions: {
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "script",
      },
    },
    files: ["src/**/*.ts", "src/**/*.tsx"],
    rules: {
      "@typescript-eslint/no-explicit-any": "off",
      "max-len": ["error", { code: 120 }],
      "prettier/prettier": [
        "warn",
        {},
        {
          usePrettierrc: true,
        },
      ],
      "react/no-unescaped-entities": "off",
    },
  },
];
