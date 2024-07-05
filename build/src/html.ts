import fs from "fs";
import path from "path";

const distPath = './dist';

fs.readdir(distPath, (err, files) =>
{
	if (err) {
		console.error('Error reading directory:', err);
		return;
	}

	const jsFiles = files.filter(file => path.extname(file) === '.js');

	jsFiles.forEach(jsFile =>
	{
		const htmlFilePath = path.join(distPath, `${path.basename(jsFile, '.js')}.html`);
		const cssFilePath = path.join(distPath, `${path.basename(jsFile, '.js')}.css`);

		let htmlContent: string = `<!DOCTYPE html>
<html lang="en">

<head>
	<meta charset="UTF-8">
	<title>${path.basename(jsFile).split('.')[0]}</title>
</head>

<body>
	<script src="./${path.basename(jsFile)}"></script>
</body>

</html>`

		fs.writeFile(htmlFilePath, htmlContent, err =>
		{
			if (err) {
				console.error(`Error creating HTML file ${htmlFilePath}:`, err);
			}
		});
	});
});
