import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import { terser } from 'rollup-plugin-terser';
import path from 'path';
import fs from 'fs';
import { sveltePreprocess } from 'svelte-preprocess';

const production = false;

export default fs
	.readdirSync(path.join(__dirname, "src", "pages"))
	.filter(file =>
	{
		const name = file.split(".")[0];
		return name != "SKIP";
	})
	.map((input) =>
	{
		const name = input.split(".")[0];
		return {
			input: "src/pages/" + input,
			output: {
				sourcemap: false,
				format: "iife",
				name: "app",
				file: "dist/" + name + ".js",
			},
			plugins: [
				svelte({
					dev: !production,
					preprocess: sveltePreprocess(),
					css: css =>
					{
						css.write(`dist/${name}.css`);
					},
					emitCss: false
				}),
				resolve({
					browser: true,
					dedupe: ['svelte'],
				}),
				commonjs(),
				typescript({
					sourceMap: !production,
					inlineSources: !production,
					tsconfig: 'tsconfig.json'
				}),
				production && terser(),
			]
		};
	});