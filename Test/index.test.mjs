import { Buffer } from "node:buffer"
import fs from "node:fs"
import Strange from "../Packages/Api"

process.loadEnvFile("./.env")

const api = new Strange(process.env.API_KEY)

const buf = api.generators.challenger({
	image: process.env.IMAGE_URL,
	silhouetted: true,
})

fs.writeFile(`${Date.now()}.png`, Buffer.from(buf), (err) => {
	if (!err) {
		// eslint-disable-next-line no-console
		console.log("Imagen guardada!")
	} else {
		console.error("Error al escribir el archivo:", err)
	}
})
