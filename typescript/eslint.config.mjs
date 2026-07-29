import v3xlabs from "eslint-plugin-v3xlabs";

// eslint-disable-next-line import/no-default-export
export default [
  { ignores: ["dist"] },
  ...v3xlabs.configs.recommended,
  { rules: { "unicorn/no-useless-undefined": "off" } },
];
