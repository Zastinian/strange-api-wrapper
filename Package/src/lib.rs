use neon::prelude::*;
use std::sync::{Arc, Mutex};
use urlencoding::encode;

mod buffers;
mod filters;
mod utils;

struct StrangeApi {
    api_key: String,
}

impl StrangeApi {
    fn new(api_key: String) -> Self {
        StrangeApi { api_key }
    }

    // FILTERS

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

    // UTILS

    pub fn circle(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::utils_buffer(
            self.api_key.clone(),
            "circle",
            params,
        ))
    }

    pub fn color(&self, code: String) -> Result<Vec<u8>, std::io::Error> {
        let params = format!("code={}", code);
        Ok(buffers::utils_buffer(self.api_key.clone(), "color", params))
    }

    pub fn denoise(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::utils_buffer(
            self.api_key.clone(),
            "denoise",
            params,
        ))
    }

    pub fn farewell_card(
        &self,
        avatar: String,
        name: String,
        discriminator: String,
        count: f64,
        guild: String,
        bkg: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoded_avatar = encode(&avatar);
        let encoded_bkg = encode(&bkg);
        let params = format!(
            "avatar={}&name={}&discriminator={}&count={}&guild={}&bkg={}",
            encoded_avatar, name, discriminator, count, guild, encoded_bkg
        );
        Ok(buffers::utils_buffer(
            self.api_key.clone(),
            "farewell-card",
            params,
        ))
    }

    pub fn rank_card(
        &self,
        avatar: String,
        currentxp: f64,
        reqxp: f64,
        level: f64,
        rank: f64,
        status: String,
        name: String,
        discriminator: String,
        barcolor: String,
        bgimage: String,
        bgcolor: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoded_avatar = encode(&avatar);
        let encoded_bgimage = encode(&bgimage);
        let params = format!(
            "avatar={}&currentxp={}&reqxp={}&level={}&rank={}&status={}&name={}&discriminator={}&barcolor={}&bgImage={}bgColor={}",
            encoded_avatar, currentxp, reqxp, level, rank, status, name, discriminator, barcolor, encoded_bgimage, bgcolor
        );
        Ok(buffers::utils_buffer(
            self.api_key.clone(),
            "rank-card",
            params,
        ))
    }

    pub fn spotify_card(
        &self,
        image: String,
        author: String,
        album: String,
        start: f64,
        end: f64,
        title: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!(
            "image={}&author={}&album={}&start={}&end={}&title={}",
            encoded_image, author, album, start, end, title
        );
        Ok(buffers::utils_buffer(
            self.api_key.clone(),
            "spotify-card",
            params,
        ))
    }

    pub fn welcome_card(
        &self,
        avatar: String,
        name: String,
        discriminator: String,
        count: f64,
        guild: String,
        bkg: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoded_avatar = encode(&avatar);
        let encoded_bkg = encode(&bkg);
        let params = format!(
            "avatar={}&name={}&discriminator={}&count={}&guild={}&bkg={}",
            encoded_avatar, name, discriminator, count, guild, encoded_bkg
        );
        Ok(buffers::utils_buffer(
            self.api_key.clone(),
            "welcome-card",
            params,
        ))
    }
}

fn init(mut cx: FunctionContext) -> JsResult<JsObject> {
    let exports = JsObject::new(&mut cx);
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);
    let strange_api = Arc::new(Mutex::new(StrangeApi::new(api_key)));

    // FILTERS

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

    // Utils

    let st = Arc::clone(&strange_api);
    let circle_utils = JsFunction::new(&mut cx, move |cx| utils::circle(cx, Arc::clone(&st)));
    exports.set(&mut cx, "circle", circle_utils?)?;

    let st = Arc::clone(&strange_api);
    let color_utils = JsFunction::new(&mut cx, move |cx| utils::color(cx, Arc::clone(&st)));
    exports.set(&mut cx, "color", color_utils?)?;

    let st = Arc::clone(&strange_api);
    let denoise_utils = JsFunction::new(&mut cx, move |cx| utils::denoise(cx, Arc::clone(&st)));
    exports.set(&mut cx, "denoise", denoise_utils?)?;

    let st = Arc::clone(&strange_api);
    let farewell_card_utils =
        JsFunction::new(&mut cx, move |cx| utils::farewell_card(cx, Arc::clone(&st)));
    exports.set(&mut cx, "farewellCard", farewell_card_utils?)?;

    let st = Arc::clone(&strange_api);
    let rank_card_utils = JsFunction::new(&mut cx, move |cx| utils::rank_card(cx, Arc::clone(&st)));
    exports.set(&mut cx, "rankCard", rank_card_utils?)?;

    let st = Arc::clone(&strange_api);
    let spotify_card_utils =
        JsFunction::new(&mut cx, move |cx| utils::spotify_card(cx, Arc::clone(&st)));
    exports.set(&mut cx, "spotifyCard", spotify_card_utils?)?;

    let st = Arc::clone(&strange_api);
    let welcome_card_utils =
        JsFunction::new(&mut cx, move |cx| utils::welcome_card(cx, Arc::clone(&st)));
    exports.set(&mut cx, "welcomeCard", welcome_card_utils?)?;

    Ok(exports)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    Ok(())
}
