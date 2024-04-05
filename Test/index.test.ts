import StrangeApi from "../Package";
import fs from "fs";
import {Buffer} from "buffer";

process.loadEnvFile("./.env");

const api = new StrangeApi(process.env.API_KEY ?? "");

const buf = api.blur({
  image: process.env.IMAGE_URL ?? "",
  level: 1,
});

fs.writeFile(`${Date.now()}.png`, Buffer.from(buf), function (err) {
  if (err) {
    console.error("Error al escribir el archivo:", err);
  } else {
    console.log("Imagen guardada!");
  }
});
