import Strange from "../Package/index.js";
import fs from "fs";
import {Buffer} from "buffer";

process.loadEnvFile("./.env");

const api = Strange(process.env.API_KEY);

const buf = api.challenger({
  image: process.env.IMAGE_URL,
  silhouetted: true,
});

fs.writeFile(`${Date.now()}.png`, Buffer.from(buf), function (err) {
  if (err) {
    console.error("Error al escribir el archivo:", err);
  } else {
    console.log("Imagen guardada!");
  }
});
