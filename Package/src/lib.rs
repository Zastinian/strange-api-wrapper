use neon::prelude::*;
use std::sync::{Arc, Mutex};
use urlencoding::encode;

mod buffers;
mod filters;
mod generators;
mod overlays;
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

    // GENERATORS

    pub fn pokemon_3000_years(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "3000-years",
            params,
        ))
    }

    pub fn ad(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "ad",
            params,
        ))
    }

    pub fn affect(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "affect",
            params,
        ))
    }

    pub fn batslap(&self, image1: String, image2: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image1 = encode(&image1);
        let encoded_image2 = encode(&image2);
        let params = format!("image1={}&image2={}", encoded_image1, encoded_image2);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "batslap",
            params,
        ))
    }

    pub fn beautiful(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "beautiful",
            params,
        ))
    }

    pub fn bed(&self, image1: String, image2: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image1 = encode(&image1);
        let encoded_image2 = encode(&image2);
        let params = format!("image1={}&image2={}", encoded_image1, encoded_image2);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "bed",
            params,
        ))
    }

    pub fn bobross(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "bobross",
            params,
        ))
    }

    pub fn challenger(&self, image: String, silhouetted: bool) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}&silhouetted={}", encoded_image, silhouetted);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "bobross",
            params,
        ))
    }

    pub fn changemymind(&self, text: String) -> Result<Vec<u8>, std::io::Error> {
        let params = format!("text={}", text);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "changemymind",
            params,
        ))
    }

    pub fn clown(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "clown",
            params,
        ))
    }

    pub fn clyde(&self, text: String) -> Result<Vec<u8>, std::io::Error> {
        let params = format!("text={}", text);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "clyde",
            params,
        ))
    }

    pub fn confusedstonk(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "confusedstonk",
            params,
        ))
    }

    pub fn delete(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "delete",
            params,
        ))
    }

    pub fn demotivational(
        &self,
        title: String,
        text: String,
        image: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("title={}&text={}&image={}", title, text, encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "demotivational",
            params,
        ))
    }

    pub fn dexter(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "dexter",
            params,
        ))
    }

    pub fn discordblack(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "discordblack",
            params,
        ))
    }

    pub fn discordblue(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "discordblue",
            params,
        ))
    }

    pub fn doublestonk(&self, image1: String, image2: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image1 = encode(&image1);
        let encoded_image2 = encode(&image2);
        let params = format!("image1={}&image2={}", encoded_image1, encoded_image2);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "doublestonk",
            params,
        ))
    }

    pub fn facepalm(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "facepalm",
            params,
        ))
    }

    pub fn fusion(&self, image1: String, image2: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image1 = encode(&image1);
        let encoded_image2 = encode(&image2);
        let params = format!("image1={}&image2={}", encoded_image1, encoded_image2);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "fusion",
            params,
        ))
    }

    pub fn gru_plan(
        &self,
        first_setp: String,
        second_step: String,
        third_step: String,
        fourth_step: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let params = format!(
            "firstSetp={}&secondStep={}&thirdStep={}&fourthStep={}",
            first_setp, second_step, third_step, fourth_step
        );
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "gru-plan",
            params,
        ))
    }

    pub fn heartbreaking(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "heartbreaking",
            params,
        ))
    }

    pub fn hitler(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "hitler",
            params,
        ))
    }

    pub fn jail(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "jail",
            params,
        ))
    }

    pub fn jokeoverhead(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "jokeoverhead",
            params,
        ))
    }

    pub fn karaba(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "karaba",
            params,
        ))
    }

    pub fn kiss(&self, image1: String, image2: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image1 = encode(&image1);
        let encoded_image2 = encode(&image2);
        let params = format!("image1={}&image2={}", encoded_image1, encoded_image2);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "kiss",
            params,
        ))
    }

    pub fn kyon_gun(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "kyon-gun",
            params,
        ))
    }

    pub fn lisa_presentation(&self, text: String) -> Result<Vec<u8>, std::io::Error> {
        let params = format!("text={}", text);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "lisa-presentation",
            params,
        ))
    }

    pub fn mikkelsen(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "mikkelsen",
            params,
        ))
    }

    pub fn mirror(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "mirror",
            params,
        ))
    }

    pub fn mms(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "mms",
            params,
        ))
    }

    pub fn notstonk(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "notstonk",
            params,
        ))
    }

    pub fn ohno(&self, text: String) -> Result<Vec<u8>, std::io::Error> {
        let params = format!("text={}", text);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "ohno",
            params,
        ))
    }

    pub fn plankton_plan(
        &self,
        first_setp: String,
        second_step: String,
        third_step: String,
        fourth_step: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let params = format!(
            "firstSetp={}&secondStep={}&thirdStep={}&fourthStep={}",
            first_setp, second_step, third_step, fourth_step
        );
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "plankton-plan",
            params,
        ))
    }

    pub fn poutine(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "poutine",
            params,
        ))
    }

    pub fn rip(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "rip",
            params,
        ))
    }

    pub fn shit(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "shit",
            params,
        ))
    }

    pub fn snyder(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "snyder",
            params,
        ))
    }

    pub fn spank(&self, image1: String, image2: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image1 = encode(&image1);
        let encoded_image2 = encode(&image2);
        let params = format!("image1={}&image2={}", encoded_image1, encoded_image2);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "spank",
            params,
        ))
    }

    pub fn stonk(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "stonk",
            params,
        ))
    }

    pub fn tattoo(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "tattoo",
            params,
        ))
    }

    pub fn thomas(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "thomas",
            params,
        ))
    }

    pub fn trash(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "trash",
            params,
        ))
    }

    pub fn wanted(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "wanted",
            params,
        ))
    }

    pub fn worthless(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "worthless",
            params,
        ))
    }

    pub fn youtube(
        &self,
        image: String,
        username: String,
        text: String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!(
            "image={}&username={}&text={}",
            encoded_image, username, text
        );
        Ok(buffers::generators_buffer(
            self.api_key.clone(),
            "youtube",
            params,
        ))
    }

    // OVERLAYS

    pub fn approved(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "approved",
            params,
        ))
    }

    pub fn brazzers(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "brazzers",
            params,
        ))
    }

    pub fn gay(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "gay",
            params,
        ))
    }

    pub fn halloween(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "halloween",
            params,
        ))
    }

    pub fn rejected(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "rejected",
            params,
        ))
    }

    pub fn thuglife(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "thuglife",
            params,
        ))
    }

    pub fn to_be_continued(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "to-be-continued",
            params,
        ))
    }

    pub fn wasted(&self, image: String) -> Result<Vec<u8>, std::io::Error> {
        let encoded_image = encode(&image);
        let params = format!("image={}", encoded_image);
        Ok(buffers::overlays_buffer(
            self.api_key.clone(),
            "wasted",
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

    // GENERATORS

    let st = Arc::clone(&strange_api);
    let pokemon_3000_years_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::pokemon_3000_years(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "pokemon3000Years", pokemon_3000_years_overlay?)?;

    let st = Arc::clone(&strange_api);
    let ad_overlay = JsFunction::new(&mut cx, move |cx| generators::ad(cx, Arc::clone(&st)));
    exports.set(&mut cx, "ad", ad_overlay?)?;

    let st = Arc::clone(&strange_api);
    let affect_overlay =
        JsFunction::new(&mut cx, move |cx| generators::affect(cx, Arc::clone(&st)));
    exports.set(&mut cx, "affect", affect_overlay?)?;

    let st = Arc::clone(&strange_api);
    let batslap_overlay =
        JsFunction::new(&mut cx, move |cx| generators::batslap(cx, Arc::clone(&st)));
    exports.set(&mut cx, "batslap", batslap_overlay?)?;

    let st = Arc::clone(&strange_api);
    let beautiful_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::beautiful(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "beautiful", beautiful_overlay?)?;

    let st = Arc::clone(&strange_api);
    let bed_overlay = JsFunction::new(&mut cx, move |cx| generators::bed(cx, Arc::clone(&st)));
    exports.set(&mut cx, "bed", bed_overlay?)?;

    let st = Arc::clone(&strange_api);
    let bobross_overlay =
        JsFunction::new(&mut cx, move |cx| generators::bobross(cx, Arc::clone(&st)));
    exports.set(&mut cx, "bobross", bobross_overlay?)?;

    let st = Arc::clone(&strange_api);
    let challenger_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::challenger(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "challenger", challenger_overlay?)?;

    let st = Arc::clone(&strange_api);
    let changemymind_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::changemymind(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "changemymind", changemymind_overlay?)?;

    let st = Arc::clone(&strange_api);
    let clown_overlay = JsFunction::new(&mut cx, move |cx| generators::clown(cx, Arc::clone(&st)));
    exports.set(&mut cx, "clown", clown_overlay?)?;

    let st = Arc::clone(&strange_api);
    let clyde_overlay = JsFunction::new(&mut cx, move |cx| generators::clyde(cx, Arc::clone(&st)));
    exports.set(&mut cx, "clyde", clyde_overlay?)?;

    let st = Arc::clone(&strange_api);
    let confusedstonk_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::confusedstonk(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "confusedstonk", confusedstonk_overlay?)?;

    let st = Arc::clone(&strange_api);
    let delete_overlay =
        JsFunction::new(&mut cx, move |cx| generators::delete(cx, Arc::clone(&st)));
    exports.set(&mut cx, "delete", delete_overlay?)?;

    let st = Arc::clone(&strange_api);
    let demotivational_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::demotivational(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "demotivational", demotivational_overlay?)?;

    let st = Arc::clone(&strange_api);
    let dexter_overlay =
        JsFunction::new(&mut cx, move |cx| generators::dexter(cx, Arc::clone(&st)));
    exports.set(&mut cx, "dexter", dexter_overlay?)?;

    let st = Arc::clone(&strange_api);
    let discordblack_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::discordblack(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "discordblack", discordblack_overlay?)?;

    let st = Arc::clone(&strange_api);
    let discordblue_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::discordblue(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "discordblue", discordblue_overlay?)?;

    let st = Arc::clone(&strange_api);
    let doublestonk_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::doublestonk(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "doublestonk", doublestonk_overlay?)?;

    let st = Arc::clone(&strange_api);
    let facepalm_overlay =
        JsFunction::new(&mut cx, move |cx| generators::facepalm(cx, Arc::clone(&st)));
    exports.set(&mut cx, "facepalm", facepalm_overlay?)?;

    let st = Arc::clone(&strange_api);
    let fusion_overlay =
        JsFunction::new(&mut cx, move |cx| generators::fusion(cx, Arc::clone(&st)));
    exports.set(&mut cx, "fusion", fusion_overlay?)?;

    let st = Arc::clone(&strange_api);
    let gru_plan_overlay =
        JsFunction::new(&mut cx, move |cx| generators::gru_plan(cx, Arc::clone(&st)));
    exports.set(&mut cx, "gruPlan", gru_plan_overlay?)?;

    let st = Arc::clone(&strange_api);
    let heartbreaking_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::heartbreaking(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "heartbreaking", heartbreaking_overlay?)?;

    let st = Arc::clone(&strange_api);
    let hitler_overlay =
        JsFunction::new(&mut cx, move |cx| generators::hitler(cx, Arc::clone(&st)));
    exports.set(&mut cx, "hitler", hitler_overlay?)?;

    let st = Arc::clone(&strange_api);
    let jail_overlay = JsFunction::new(&mut cx, move |cx| generators::jail(cx, Arc::clone(&st)));
    exports.set(&mut cx, "jail", jail_overlay?)?;

    let st = Arc::clone(&strange_api);
    let jokeoverhead_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::jokeoverhead(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "jokeoverhead", jokeoverhead_overlay?)?;

    let st = Arc::clone(&strange_api);
    let karaba_overlay =
        JsFunction::new(&mut cx, move |cx| generators::karaba(cx, Arc::clone(&st)));
    exports.set(&mut cx, "karaba", karaba_overlay?)?;

    let st = Arc::clone(&strange_api);
    let kiss_overlay = JsFunction::new(&mut cx, move |cx| generators::kiss(cx, Arc::clone(&st)));
    exports.set(&mut cx, "kiss", kiss_overlay?)?;

    let st = Arc::clone(&strange_api);
    let kyon_gun_overlay =
        JsFunction::new(&mut cx, move |cx| generators::kyon_gun(cx, Arc::clone(&st)));
    exports.set(&mut cx, "kyonGun", kyon_gun_overlay?)?;

    let st = Arc::clone(&strange_api);
    let lisa_presentation_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::lisa_presentation(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "lisaPresentation", lisa_presentation_overlay?)?;

    let st = Arc::clone(&strange_api);
    let mikkelsen_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::mikkelsen(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "mikkelsen", mikkelsen_overlay?)?;

    let st = Arc::clone(&strange_api);
    let mirror_overlay =
        JsFunction::new(&mut cx, move |cx| generators::mirror(cx, Arc::clone(&st)));
    exports.set(&mut cx, "mirror", mirror_overlay?)?;

    let st = Arc::clone(&strange_api);
    let mms_overlay = JsFunction::new(&mut cx, move |cx| generators::mms(cx, Arc::clone(&st)));
    exports.set(&mut cx, "mms", mms_overlay?)?;

    let st = Arc::clone(&strange_api);
    let notstonk_overlay =
        JsFunction::new(&mut cx, move |cx| generators::notstonk(cx, Arc::clone(&st)));
    exports.set(&mut cx, "notstonk", notstonk_overlay?)?;

    let st = Arc::clone(&strange_api);
    let ohno_overlay = JsFunction::new(&mut cx, move |cx| generators::ohno(cx, Arc::clone(&st)));
    exports.set(&mut cx, "ohno", ohno_overlay?)?;

    let st = Arc::clone(&strange_api);
    let plankton_plan_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::plankton_plan(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "planktonPlan", plankton_plan_overlay?)?;

    let st = Arc::clone(&strange_api);
    let poutine_overlay =
        JsFunction::new(&mut cx, move |cx| generators::poutine(cx, Arc::clone(&st)));
    exports.set(&mut cx, "poutine", poutine_overlay?)?;

    let st = Arc::clone(&strange_api);
    let rip_overlay = JsFunction::new(&mut cx, move |cx| generators::rip(cx, Arc::clone(&st)));
    exports.set(&mut cx, "rip", rip_overlay?)?;

    let st = Arc::clone(&strange_api);
    let shit_overlay = JsFunction::new(&mut cx, move |cx| generators::shit(cx, Arc::clone(&st)));
    exports.set(&mut cx, "shit", shit_overlay?)?;

    let st = Arc::clone(&strange_api);
    let snyder_overlay =
        JsFunction::new(&mut cx, move |cx| generators::snyder(cx, Arc::clone(&st)));
    exports.set(&mut cx, "snyder", snyder_overlay?)?;

    let st = Arc::clone(&strange_api);
    let spank_overlay = JsFunction::new(&mut cx, move |cx| generators::spank(cx, Arc::clone(&st)));
    exports.set(&mut cx, "spank", spank_overlay?)?;

    let st = Arc::clone(&strange_api);
    let stonk_overlay = JsFunction::new(&mut cx, move |cx| generators::stonk(cx, Arc::clone(&st)));
    exports.set(&mut cx, "stonk", stonk_overlay?)?;

    let st = Arc::clone(&strange_api);
    let tattoo_overlay =
        JsFunction::new(&mut cx, move |cx| generators::tattoo(cx, Arc::clone(&st)));
    exports.set(&mut cx, "tattoo", tattoo_overlay?)?;

    let st = Arc::clone(&strange_api);
    let thomas_overlay =
        JsFunction::new(&mut cx, move |cx| generators::thomas(cx, Arc::clone(&st)));
    exports.set(&mut cx, "thomas", thomas_overlay?)?;

    let st = Arc::clone(&strange_api);
    let trash_overlay = JsFunction::new(&mut cx, move |cx| generators::trash(cx, Arc::clone(&st)));
    exports.set(&mut cx, "trash", trash_overlay?)?;

    let st = Arc::clone(&strange_api);
    let wanted_overlay =
        JsFunction::new(&mut cx, move |cx| generators::wanted(cx, Arc::clone(&st)));
    exports.set(&mut cx, "wanted", wanted_overlay?)?;

    let st = Arc::clone(&strange_api);
    let worthless_overlay = JsFunction::new(&mut cx, move |cx| {
        generators::worthless(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "worthless", worthless_overlay?)?;

    let st = Arc::clone(&strange_api);
    let youtube_overlay =
        JsFunction::new(&mut cx, move |cx| generators::youtube(cx, Arc::clone(&st)));
    exports.set(&mut cx, "youtube", youtube_overlay?)?;

    // OVERLAYS

    let st = Arc::clone(&strange_api);
    let approved_overlay =
        JsFunction::new(&mut cx, move |cx| overlays::approved(cx, Arc::clone(&st)));
    exports.set(&mut cx, "approved", approved_overlay?)?;

    let st = Arc::clone(&strange_api);
    let brazzers_overlay =
        JsFunction::new(&mut cx, move |cx| overlays::brazzers(cx, Arc::clone(&st)));
    exports.set(&mut cx, "brazzers", brazzers_overlay?)?;

    let st = Arc::clone(&strange_api);
    let gay_overlay = JsFunction::new(&mut cx, move |cx| overlays::gay(cx, Arc::clone(&st)));
    exports.set(&mut cx, "gay", gay_overlay?)?;

    let st = Arc::clone(&strange_api);
    let halloween_overlay =
        JsFunction::new(&mut cx, move |cx| overlays::halloween(cx, Arc::clone(&st)));
    exports.set(&mut cx, "halloween", halloween_overlay?)?;

    let st = Arc::clone(&strange_api);
    let rejected_overlay =
        JsFunction::new(&mut cx, move |cx| overlays::rejected(cx, Arc::clone(&st)));
    exports.set(&mut cx, "rejected", rejected_overlay?)?;

    let st = Arc::clone(&strange_api);
    let thuglife_overlay =
        JsFunction::new(&mut cx, move |cx| overlays::thuglife(cx, Arc::clone(&st)));
    exports.set(&mut cx, "thuglife", thuglife_overlay?)?;

    let st = Arc::clone(&strange_api);
    let to_be_continued_overlay = JsFunction::new(&mut cx, move |cx| {
        overlays::to_be_continued(cx, Arc::clone(&st))
    });
    exports.set(&mut cx, "toBeContinued", to_be_continued_overlay?)?;

    let st = Arc::clone(&strange_api);
    let wasted_overlay = JsFunction::new(&mut cx, move |cx| overlays::wasted(cx, Arc::clone(&st)));
    exports.set(&mut cx, "wasted", wasted_overlay?)?;

    // UTILS

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
