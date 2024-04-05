use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

use crate::StrangeApi;

pub fn circle(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.circle(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn color(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let code_data: Handle<JsValue> = data.get(&mut cx, "code")?;
    let code = code_data.to_string(&mut cx)?.value(&mut cx);
    if code == "undefined" {
        return cx.throw_error("'code' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.color(code) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn denoise(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.denoise(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn farewell_card(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let avatar_data: Handle<JsValue> = data.get(&mut cx, "avatar")?;
    let avatar = avatar_data.to_string(&mut cx)?.value(&mut cx);
    if avatar == "undefined" {
        return cx.throw_error("'avatar' field is missing");
    }
    let name_data: Handle<JsValue> = data.get(&mut cx, "name")?;
    let name = name_data.to_string(&mut cx)?.value(&mut cx);
    if name == "undefined" {
        return cx.throw_error("'name' field is missing");
    }
    let discriminator_data: Handle<JsValue> = data.get(&mut cx, "discriminator")?;
    let discriminator = discriminator_data.to_string(&mut cx)?.value(&mut cx);
    if discriminator == "undefined" {
        return cx.throw_error("'discriminator' field is missing");
    }
    let count_data: Handle<JsValue> = data.get(&mut cx, "count")?;
    let count_string = count_data.to_string(&mut cx)?.value(&mut cx);
    if count_string == "undefined" {
        return cx.throw_error("'count' field is missing");
    }
    let count: f64 = count_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let guild_data: Handle<JsValue> = data.get(&mut cx, "guild")?;
    let guild = guild_data.to_string(&mut cx)?.value(&mut cx);
    if guild == "undefined" {
        return cx.throw_error("'guild' field is missing");
    }
    let bkg_data: Handle<JsValue> = data.get(&mut cx, "bkg")?;
    let bkg = bkg_data.to_string(&mut cx)?.value(&mut cx);
    if bkg == "undefined" {
        return cx.throw_error("'bkg' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.farewell_card(avatar, name, discriminator, count, guild, bkg) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn rank_card(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let avatar_data: Handle<JsValue> = data.get(&mut cx, "avatar")?;
    let avatar = avatar_data.to_string(&mut cx)?.value(&mut cx);
    if avatar == "undefined" {
        return cx.throw_error("'avatar' field is missing");
    }
    let currentxp_data: Handle<JsValue> = data.get(&mut cx, "currentxp")?;
    let currentxp_string = currentxp_data.to_string(&mut cx)?.value(&mut cx);
    if currentxp_string == "undefined" {
        return cx.throw_error("'currentxp' field is missing");
    }
    let currentxp: f64 = currentxp_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let reqxp_data: Handle<JsValue> = data.get(&mut cx, "reqxp")?;
    let reqxp_string = reqxp_data.to_string(&mut cx)?.value(&mut cx);
    if reqxp_string == "undefined" {
        return cx.throw_error("'reqxp' field is missing");
    }
    let reqxp: f64 = reqxp_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let level_data: Handle<JsValue> = data.get(&mut cx, "level")?;
    let level_string = level_data.to_string(&mut cx)?.value(&mut cx);
    if level_string == "undefined" {
        return cx.throw_error("'level' field is missing");
    }
    let level: f64 = level_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let rank_data: Handle<JsValue> = data.get(&mut cx, "rank")?;
    let rank_string = rank_data.to_string(&mut cx)?.value(&mut cx);
    if rank_string == "undefined" {
        return cx.throw_error("'rank' field is missing");
    }
    let rank: f64 = rank_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let status_data: Handle<JsValue> = data.get(&mut cx, "status")?;
    let status = status_data.to_string(&mut cx)?.value(&mut cx);
    if status == "undefined" {
        return cx.throw_error("'status' field is missing");
    }
    let name_data: Handle<JsValue> = data.get(&mut cx, "name")?;
    let name = name_data.to_string(&mut cx)?.value(&mut cx);
    if name == "undefined" {
        return cx.throw_error("'name' field is missing");
    }
    let discriminator_data: Handle<JsValue> = data.get(&mut cx, "discriminator")?;
    let discriminator = discriminator_data.to_string(&mut cx)?.value(&mut cx);
    if discriminator == "undefined" {
        return cx.throw_error("'discriminator' field is missing");
    }
    let barcolor_data: Handle<JsValue> = data.get(&mut cx, "barcolor")?;
    let barcolor = barcolor_data.to_string(&mut cx)?.value(&mut cx);
    if barcolor == "undefined" {
        return cx.throw_error("'barcolor' field is missing");
    }
    let bgimage_data: Handle<JsValue> = data.get(&mut cx, "bgimage")?;
    let bgimage = bgimage_data.to_string(&mut cx)?.value(&mut cx);
    if bgimage == "undefined" {
        return cx.throw_error("'avatar' field is missing");
    }
    let bgcolor_data: Handle<JsValue> = data.get(&mut cx, "bgcolor")?;
    let bgcolor = bgcolor_data.to_string(&mut cx)?.value(&mut cx);
    if bgcolor == "undefined" {
        return cx.throw_error("'bgcolor' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.rank_card(
        avatar,
        currentxp,
        reqxp,
        level,
        rank,
        status,
        name,
        discriminator,
        barcolor,
        bgimage,
        bgcolor,
    ) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn spotify_card(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let author_data: Handle<JsValue> = data.get(&mut cx, "author")?;
    let author = author_data.to_string(&mut cx)?.value(&mut cx);
    if author == "undefined" {
        return cx.throw_error("'author' field is missing");
    }
    let album_data: Handle<JsValue> = data.get(&mut cx, "album")?;
    let album = album_data.to_string(&mut cx)?.value(&mut cx);
    if album == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let start_data: Handle<JsValue> = data.get(&mut cx, "start")?;
    let start_string = start_data.to_string(&mut cx)?.value(&mut cx);
    if start_string == "undefined" {
        return cx.throw_error("'start' field is missing");
    }
    let start: f64 = start_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let end_data: Handle<JsValue> = data.get(&mut cx, "end")?;
    let end_string = end_data.to_string(&mut cx)?.value(&mut cx);
    if end_string == "undefined" {
        return cx.throw_error("'end' field is missing");
    }
    let end: f64 = end_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let title_data: Handle<JsValue> = data.get(&mut cx, "title")?;
    let title = title_data.to_string(&mut cx)?.value(&mut cx);
    if title == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.spotify_card(image, author, album, start, end, title) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn welcome_card(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let avatar_data: Handle<JsValue> = data.get(&mut cx, "avatar")?;
    let avatar = avatar_data.to_string(&mut cx)?.value(&mut cx);
    if avatar == "undefined" {
        return cx.throw_error("'avatar' field is missing");
    }
    let name_data: Handle<JsValue> = data.get(&mut cx, "name")?;
    let name = name_data.to_string(&mut cx)?.value(&mut cx);
    if name == "undefined" {
        return cx.throw_error("'name' field is missing");
    }
    let discriminator_data: Handle<JsValue> = data.get(&mut cx, "discriminator")?;
    let discriminator = discriminator_data.to_string(&mut cx)?.value(&mut cx);
    if discriminator == "undefined" {
        return cx.throw_error("'discriminator' field is missing");
    }
    let count_data: Handle<JsValue> = data.get(&mut cx, "count")?;
    let count_string = count_data.to_string(&mut cx)?.value(&mut cx);
    if count_string == "undefined" {
        return cx.throw_error("'count' field is missing");
    }
    let count: f64 = count_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let guild_data: Handle<JsValue> = data.get(&mut cx, "guild")?;
    let guild = guild_data.to_string(&mut cx)?.value(&mut cx);
    if guild == "undefined" {
        return cx.throw_error("'guild' field is missing");
    }
    let bkg_data: Handle<JsValue> = data.get(&mut cx, "bkg")?;
    let bkg = bkg_data.to_string(&mut cx)?.value(&mut cx);
    if bkg == "undefined" {
        return cx.throw_error("'bkg' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.welcome_card(avatar, name, discriminator, count, guild, bkg) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}
