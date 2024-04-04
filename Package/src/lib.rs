use neon::prelude::*;
use std::sync::{Arc, Mutex};
use urlencoding::encode;

mod buffers;
mod filters;

struct StrangeApi {
    api_key: String,
}

impl StrangeApi {
    fn new(api_key: String) -> Self {
        StrangeApi { api_key }
    }

    pub fn blur(&self, image: String, level: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&level={}", encoded_image, level);
        Ok(buffers::filter_buffer(self.api_key.clone(), "blur", params))
    }

    pub fn brighten(&self, image: String, amount: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&amount={}", encoded_image, amount);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "brighten",
            params,
        ))
    }

    pub fn burn(&self, image: String, level: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&level={}", encoded_image, level);
        Ok(buffers::filter_buffer(self.api_key.clone(), "burn", params))
    }

    pub fn darken(&self, image: String, amount: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&amount={}", encoded_image, amount);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "darken",
            params,
        ))
    }

    pub fn deepfry(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "deepfry",
            params,
        ))
    }

    pub fn distort(&self, image: String, level: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&level={}", encoded_image, level);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "distort",
            params,
        ))
    }

    pub fn greyscale(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "greyscale",
            params,
        ))
    }

    pub fn invert(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "invert",
            params,
        ))
    }

    pub fn pixelate(&self, image: String, pixels: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&pixels={}", encoded_image, pixels);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "pixelate",
            params,
        ))
    }

    pub fn sepia(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "sepia",
            params,
        ))
    }

    pub fn sharpen(&self, image: String, level: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&level={}", encoded_image, level);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "sharpen",
            params,
        ))
    }

    pub fn threshold(&self, image: String, amount: f64) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&amount={}", encoded_image, amount);
        Ok(buffers::filter_buffer(
            self.api_key.clone(),
            "threshold",
            params,
        ))
    }
}

fn init(mut cx: FunctionContext) -> JsResult<JsObject> {
    let exports = JsObject::new(&mut cx);
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let strange_api = Arc::new(Mutex::new(StrangeApi::new(api_key)));

    let st = Arc::clone(&strange_api);
    let blur_filter = JsFunction::new(&mut cx, move |cx| filters::blur(cx, Arc::clone(&st)));
    exports.set(&mut cx, "blur", blur_filter?)?;

    let st = Arc::clone(&strange_api);
    let brighten_filter =
        JsFunction::new(&mut cx, move |cx| filters::brighten(cx, Arc::clone(&st)));
    exports.set(&mut cx, "brighten", brighten_filter?)?;

    let st = Arc::clone(&strange_api);
    let burn_filter = JsFunction::new(&mut cx, move |cx| filters::burn(cx, Arc::clone(&st)));
    exports.set(&mut cx, "burn", burn_filter?)?;

    let st = Arc::clone(&strange_api);
    let darken_filter = JsFunction::new(&mut cx, move |cx| filters::darken(cx, Arc::clone(&st)));
    exports.set(&mut cx, "darken", darken_filter?)?;

    let st = Arc::clone(&strange_api);
    let deepfry_filter = JsFunction::new(&mut cx, move |cx| filters::deepfry(cx, Arc::clone(&st)));
    exports.set(&mut cx, "deepfry", deepfry_filter?)?;

    let st = Arc::clone(&strange_api);
    let distort_filter = JsFunction::new(&mut cx, move |cx| filters::distort(cx, Arc::clone(&st)));
    exports.set(&mut cx, "distort", distort_filter?)?;

    let st = Arc::clone(&strange_api);
    let greyscale_filter =
        JsFunction::new(&mut cx, move |cx| filters::greyscale(cx, Arc::clone(&st)));
    exports.set(&mut cx, "greyscale", greyscale_filter?)?;

    let st = Arc::clone(&strange_api);
    let invert_filter = JsFunction::new(&mut cx, move |cx| filters::invert(cx, Arc::clone(&st)));
    exports.set(&mut cx, "invert", invert_filter?)?;

    let st = Arc::clone(&strange_api);
    let pixelate_filter =
        JsFunction::new(&mut cx, move |cx| filters::pixelate(cx, Arc::clone(&st)));
    exports.set(&mut cx, "pixelate", pixelate_filter?)?;

    let st = Arc::clone(&strange_api);
    let sepia_filter = JsFunction::new(&mut cx, move |cx| filters::sepia(cx, Arc::clone(&st)));
    exports.set(&mut cx, "sepia", sepia_filter?)?;

    let st = Arc::clone(&strange_api);
    let sharpen_filter = JsFunction::new(&mut cx, move |cx| filters::sharpen(cx, Arc::clone(&st)));
    exports.set(&mut cx, "sharpen", sharpen_filter?)?;

    let st = Arc::clone(&strange_api);
    let threshold_filter =
        JsFunction::new(&mut cx, move |cx| filters::threshold(cx, Arc::clone(&st)));
    exports.set(&mut cx, "threshold", threshold_filter?)?;

    Ok(exports)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    Ok(())
}
