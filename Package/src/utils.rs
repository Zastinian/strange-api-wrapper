use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

use crate::StrangeApi;

pub fn circle(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let code = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let avatar = cx.argument::<JsString>(0)?.value(&mut cx);
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    let discriminator = cx.argument::<JsString>(2)?.value(&mut cx);
    let count = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let guild = cx.argument::<JsString>(4)?.value(&mut cx);
    let bkg = cx.argument::<JsString>(5)?.value(&mut cx);
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
    let avatar = cx.argument::<JsString>(0)?.value(&mut cx);
    let currentxp = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let reqxp = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let level = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let rank = cx.argument::<JsNumber>(4)?.value(&mut cx);
    let status = cx.argument::<JsString>(5)?.value(&mut cx);
    let name = cx.argument::<JsString>(6)?.value(&mut cx);
    let discriminator = cx.argument::<JsString>(7)?.value(&mut cx);
    let barcolor = cx.argument::<JsString>(8)?.value(&mut cx);
    let bgimage = cx.argument::<JsString>(9)?.value(&mut cx);
    let bgcolor = cx.argument::<JsString>(10)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let author = cx.argument::<JsString>(1)?.value(&mut cx);
    let album = cx.argument::<JsString>(2)?.value(&mut cx);
    let start = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let end = cx.argument::<JsNumber>(4)?.value(&mut cx);
    let title = cx.argument::<JsString>(5)?.value(&mut cx);
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
    let avatar = cx.argument::<JsString>(0)?.value(&mut cx);
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    let discriminator = cx.argument::<JsString>(2)?.value(&mut cx);
    let count = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let guild = cx.argument::<JsString>(4)?.value(&mut cx);
    let bkg = cx.argument::<JsString>(5)?.value(&mut cx);
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
