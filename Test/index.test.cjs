const { Buffer } = require("node:buffer");
const fs = require("node:fs");
const { default: StrangeApi } = require("../Packages/Api");

process.loadEnvFile("./.env");

const api = new StrangeApi(process.env.API_KEY);

const buf = api.generators.challenger({
	image: process.env.IMAGE_URL,
	silhouetted: true,
});

fs.writeFile(`${Date.now()}.png`, Buffer.from(buf), (err) => {
	if (!err) {
		console.info("Imagen guardada!");
	} else {
		console.error("Error al escribir el archivo:", err);
	}
});
