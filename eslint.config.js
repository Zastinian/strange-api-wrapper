import ts from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import eslintConfigPrettier from "eslint-config-prettier";
import functional from "eslint-plugin-functional";
import imprt from "eslint-plugin-import";

export default [
	eslintConfigPrettier,
	{
		files: ["src/**/*.{ts,js,tsx,jsx}"],
		plugins: {
			functional,
			"import": imprt,
			"@typescript-eslint": ts,
			ts,
		},
		languageOptions: {
			parser: tsParser,
			parserOptions: {
				ecmaFeatures: { modules: true },
				ecmaVersion: "latest",
				project: "./tsconfig.json",
			},
		},
		rules: {
			...ts.configs["eslint-recommended"].rules,
			...ts.configs["recommended"].rules,
			"ts/return-await": "off",
			"eol-last": "off",
			"quotes": "off",
			"semi": ["warn", "never"],
			"@stylistic/js/no-tabs": "off",
			"@stylistic/ts/indent": "off",
			"no-constant-binary-expression": "warn",
			"no-undef": "off",
			"no-console": "off",
			"no-debugger": "warn",
			"no-sequences": "off",
			"no-import-assign": "off",
			"no-extend-native": "off",
			"no-trailing-spaces": "warn",
			"no-case-declarations": "off",
			"no-prototype-builtins": "off",
			"no-unused-expressions": "off",
			"space-before-function-paren": "off",
			"antfu/if-newline": "off",
			"antfu/top-level-function": "off",
			"@stylistic/js/operator-linebreak": "off",
			"@stylistic/ts/brace-style": "off",
			"@stylistic/js/multiline-ternary": "off",
			"n/prefer-global/process": "off",
			"@stylistic/js/no-mixed-spaces-and-tabs": "off",
			"no-unused-vars": "off",
			"unused-imports/no-unused-vars": "off",
			"@typescript-eslint/require-await": "off",
			"@typescript-eslint/no-unsafe-call": "off",
			"@typescript-eslint/no-explicit-any": "off",
			"@typescript-eslint/no-invalid-this": "off",
			"@typescript-eslint/no-unsafe-return": "off",
			"@typescript-eslint/no-unsafe-argument": "off",
			"@typescript-eslint/no-misused-promises": "off",
			"@typescript-eslint/no-unsafe-assignment": "off",
			"@typescript-eslint/no-unsafe-member-access": "off",
			"object-curly-newline": [
				"warn",
				{
					consistent: true,
					multiline: true,
				},
			],
			"object-curly-spacing": ["warn", "always"],
			"array-element-newline": ["warn", "consistent"],
			"array-bracket-newline": ["warn", "consistent"],
		},
	},
];
