use urlencoding::encode;

use crate::buffers;


pub struct StrangeApi {
    api_key: String,
}

impl StrangeApi {
    pub fn new(api_key: String) -> Self {
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