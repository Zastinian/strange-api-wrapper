/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { Buffer } from "node:buffer"
import fs from "node:fs"
import StrangeApi from "../Packages/Api"

process.loadEnvFile("./.env")

// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
const api = new StrangeApi(process.env.API_KEY ?? "")

// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
const buf = api.generators.challenger({
	// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
	image: process.env.IMAGE_URL ?? "",
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
