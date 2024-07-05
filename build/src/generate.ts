import fs from "fs";
import path from "path";

const boilerplate: string = `import App from "../svelte/%APPNAME.svelte";

const app = new App({
  target: document.body,
});

export default app;`

fs.readdirSync("src/svelte").forEach((file) =>
{
	const name = path.basename(file).split(".")[0];
	if (name != "SKIP") {
		fs.writeFileSync(`src/pages/${name}.ts`, boilerplate.replaceAll('%APPNAME', name));
	}
});