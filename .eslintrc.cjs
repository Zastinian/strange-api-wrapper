process.env.ESLINT_TSCONFIG = "tsconfig.json"

/**
 * @type {import("eslint").Linter.Config}
 */
module.exports = {
	globals: {
		NodeJS: true,
		NodeListOf: true,
	},
	env: {
		es2022: true,
		node: true,
		browser: true,
	},
	extends: ["@antfu", "eslint-config-prettier"],
	parser: "@typescript-eslint/parser",
	parserOptions: {
		ecmaVersion: "latest",
		sourceType: "module",
	},
	rules: {
		"eol-last": "off",
		"quotes": ["warn", "double"],
		"semi": ["warn", "never"],
		"@stylistic/js/no-tabs": "off",
		"@stylistic/ts/indent": "off",
		"no-constant-binary-expression": "warn",
		"no-console": "off",
		"no-debugger": "warn",
		"no-import-assign": "off",
		"no-extend-native": "off",
		"no-trailing-spaces": "warn",
		"space-before-function-paren": "off",
		"antfu/if-newline": "off",
		"antfu/top-level-function": "off",
		"@stylistic/js/operator-linebreak": "off",
		"@stylistic/ts/brace-style": "off",
		"@stylistic/js/multiline-ternary": "off",
		"n/prefer-global/process": "off",
		"@stylistic/js/no-mixed-spaces-and-tabs": "off",
		"no-unused-vars": "off",
		"@typescript-eslint/no-unused-vars": "off",
		"@typescript-eslint/no-unsafe-call": "off",
		"@typescript-eslint/no-unsafe-argument": "off",
		"@typescript-eslint/no-unsafe-assignment": "off",
		"@typescript-eslint/no-unsafe-member-access": "off",
		"unused-imports/no-unused-imports": "warn",
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
	overrides: [
		{
			files: ["**/*.js", "*.js"],
			parser: "@typescript-eslint/parser",
		},
	],
}
