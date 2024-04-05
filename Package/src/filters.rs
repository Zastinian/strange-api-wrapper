use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

use crate::StrangeApi;

pub fn blur(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let level_data: Handle<JsValue> = data.get(&mut cx, "level")?;
    let level_string = level_data.to_string(&mut cx)?.value(&mut cx);
    if level_string == "undefined" {
        return cx.throw_error("'level' field is missing");
    }
    let level: f64 = level_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.blur(image, level) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn brighten(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let amount_data: Handle<JsValue> = data.get(&mut cx, "amount")?;
    let amount_string = amount_data.to_string(&mut cx)?.value(&mut cx);
    if amount_string == "undefined" {
        return cx.throw_error("'amount' field is missing");
    }
    let amount: f64 = amount_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.brighten(image, amount) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn burn(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let level_data: Handle<JsValue> = data.get(&mut cx, "level")?;
    let level_string = level_data.to_string(&mut cx)?.value(&mut cx);
    if level_string == "undefined" {
        return cx.throw_error("'level' field is missing");
    }
    let level: f64 = level_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.burn(image, level) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn darken(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let amount_data: Handle<JsValue> = data.get(&mut cx, "amount")?;
    let amount_string = amount_data.to_string(&mut cx)?.value(&mut cx);
    if amount_string == "undefined" {
        return cx.throw_error("'amount' field is missing");
    }
    let amount: f64 = amount_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.darken(image, amount) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn deepfry(
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
    match st.deepfry(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn distort(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let level_data: Handle<JsValue> = data.get(&mut cx, "level")?;
    let level_string = level_data.to_string(&mut cx)?.value(&mut cx);
    if level_string == "undefined" {
        return cx.throw_error("'level' field is missing");
    }
    let level: f64 = level_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.distort(image, level) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn greyscale(
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
    match st.greyscale(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn invert(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.invert(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn pixelate(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let pixels_data: Handle<JsValue> = match data.get(&mut cx, "pixels") {
        Ok(value) => value,
        Err(_) => return cx.throw_error("'pixels' field is missing"),
    };
    let pixels_string = pixels_data.to_string(&mut cx)?.value(&mut cx);
    if pixels_string == "undefined" {
        return cx.throw_error("'pixels' field is missing");
    }
    let pixels: f64 = pixels_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.pixelate(image, pixels) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn sepia(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.sepia(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn sharpen(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let level_data: Handle<JsValue> = data.get(&mut cx, "level")?;
    let level_string = level_data.to_string(&mut cx)?.value(&mut cx);
    if level_string == "undefined" {
        return cx.throw_error("'level' field is missing");
    }
    let level: f64 = level_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.sharpen(image, level) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn threshold(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let amount_data: Handle<JsValue> = data.get(&mut cx, "amount")?;
    let amount_string = amount_data.to_string(&mut cx)?.value(&mut cx);
    if amount_string == "undefined" {
        return cx.throw_error("'amount' field is missing");
    }
    let amount: f64 = amount_data
        .downcast_or_throw::<JsNumber, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.threshold(image, amount) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}
