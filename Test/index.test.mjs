import Strange from "../Package/index.js";
import fs from "fs";
import {Buffer} from "buffer";

const api = Strange("token");

const blur = api.pixelate("image", 32);

fs.writeFile("img.png", Buffer.from(blur), function (err) {
  if (err) {
    console.error("Error al escribir el archivo:", err);
  } else {
    console.log("Imagen guardada correctamente en", "rutaImagen");
  }
});
