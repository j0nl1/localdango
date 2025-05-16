import { defineConfig } from "@rslib/core";

export default defineConfig({
	lib: [
		{
			format: "esm",
			syntax: "es2021",
			dts: {
				bundle: false,
				distPath: "./build",
			},
			bundle: false,
		},
		{ format: "cjs", syntax: "es2021", bundle: false },
	],

	output: {
		distPath: {
			root: "./build/src",
		},
		target: "node",
		copy: [
			{
				from: "../target/debug/localdango",
				to: "./bin",
			},
		],
	},
});
