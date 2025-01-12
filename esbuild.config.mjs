import builtins from "builtin-modules";
import esbuild from "esbuild";
import esbuildSvelte from "esbuild-svelte";
import process from "process";
import { sveltePreprocess } from "svelte-preprocess";
import { wasmPluginEsbuild } from "./wasmPlugin";

const banner = `/*
THIS IS A GENERATED/BUNDLED FILE BY ESBUILD
if you want to view the source, please visit the github repository of this plugin
*/
`;

const prod = process.argv[2] === "production";

const context = await esbuild.context({
	banner: {
		js: banner,
	},
	entryPoints: ["src/main.ts"],
	bundle: true,
	external: [
		"obsidian",
		"electron",
		"@codemirror/autocomplete",
		"@codemirror/collab",
		"@codemirror/commands",
		"@codemirror/language",
		"@codemirror/lint",
		"@codemirror/search",
		"@codemirror/state",
		"@codemirror/view",
		"@lezer/common",
		"@lezer/highlight",
		"@lezer/lr",
		...builtins,
	],
	format: "cjs",
	target: "es2018",
	logLevel: "info",
	sourcemap: prod ? false : "inline",
	minify: prod,
	treeShaking: true,
	outfile: "main.js",
	conditions: ['browser', prod ? "production" : "development"],
	plugins: [
		esbuildSvelte({
			cache: false,
			compilerOptions: { css: 'injected', dev: !prod },
			preprocess: sveltePreprocess(),
		}),
		wasmPluginEsbuild,
	],
});

if (prod) {
	await context.rebuild();
	process.exit(0);
} else {
	await context.watch();
}
