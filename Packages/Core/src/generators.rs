use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

use crate::client::StrangeApi;

pub fn pokemon_3000_years(
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
    match st.pokemon_3000_years(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn ad(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.ad(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn affect(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.affect(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn batslap(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image1_data: Handle<JsValue> = data.get(&mut cx, "image1")?;
    let image1 = image1_data.to_string(&mut cx)?.value(&mut cx);
    if image1 == "undefined" {
        return cx.throw_error("'image1' field is missing");
    }
    let image2_data: Handle<JsValue> = data.get(&mut cx, "image2")?;
    let image2 = image2_data.to_string(&mut cx)?.value(&mut cx);
    if image2 == "undefined" {
        return cx.throw_error("'image2' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.batslap(image1, image2) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn beautiful(
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
    match st.beautiful(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn bed(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image1_data: Handle<JsValue> = data.get(&mut cx, "image1")?;
    let image1 = image1_data.to_string(&mut cx)?.value(&mut cx);
    if image1 == "undefined" {
        return cx.throw_error("'image1' field is missing");
    }
    let image2_data: Handle<JsValue> = data.get(&mut cx, "image2")?;
    let image2 = image2_data.to_string(&mut cx)?.value(&mut cx);
    if image2 == "undefined" {
        return cx.throw_error("'image2' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.bed(image1, image2) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn bobross(
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
    match st.bobross(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn challenger(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let silhouetted_data: Handle<JsValue> = data.get(&mut cx, "silhouetted")?;
    let silhouetted_string = silhouetted_data.to_string(&mut cx)?.value(&mut cx);
    if silhouetted_string == "undefined" {
        return cx.throw_error("'silhouetted' field is missing");
    }
    let silhouetted: bool = silhouetted_data
        .downcast_or_throw::<JsBoolean, _>(&mut cx)?
        .value(&mut cx);
    let st = strange.lock().unwrap();
    match st.challenger(image, silhouetted) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn changemymind(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let text_data: Handle<JsValue> = data.get(&mut cx, "text")?;
    let text = text_data.to_string(&mut cx)?.value(&mut cx);
    if text == "undefined" {
        return cx.throw_error("'text' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.changemymind(text) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn clown(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.clown(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn clyde(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let text_data: Handle<JsValue> = data.get(&mut cx, "text")?;
    let text = text_data.to_string(&mut cx)?.value(&mut cx);
    if text == "undefined" {
        return cx.throw_error("'text' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.clyde(text) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn confusedstonk(
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
    match st.confusedstonk(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn delete(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.delete(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn demotivational(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let title_data: Handle<JsValue> = data.get(&mut cx, "title")?;
    let title = title_data.to_string(&mut cx)?.value(&mut cx);
    if title == "undefined" {
        return cx.throw_error("'title' field is missing");
    }
    let text_data: Handle<JsValue> = data.get(&mut cx, "text")?;
    let text = text_data.to_string(&mut cx)?.value(&mut cx);
    if text == "undefined" {
        return cx.throw_error("'text' field is missing");
    }
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.demotivational(title, text, image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn dexter(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.dexter(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn discordblack(
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
    match st.discordblack(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn discordblue(
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
    match st.discordblue(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn doublestonk(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image1_data: Handle<JsValue> = data.get(&mut cx, "image1")?;
    let image1 = image1_data.to_string(&mut cx)?.value(&mut cx);
    if image1 == "undefined" {
        return cx.throw_error("'image1' field is missing");
    }
    let image2_data: Handle<JsValue> = data.get(&mut cx, "image2")?;
    let image2 = image2_data.to_string(&mut cx)?.value(&mut cx);
    if image2 == "undefined" {
        return cx.throw_error("'image2' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.doublestonk(image1, image2) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn facepalm(
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
    match st.facepalm(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn fusion(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image1_data: Handle<JsValue> = data.get(&mut cx, "image1")?;
    let image1 = image1_data.to_string(&mut cx)?.value(&mut cx);
    if image1 == "undefined" {
        return cx.throw_error("'image1' field is missing");
    }
    let image2_data: Handle<JsValue> = data.get(&mut cx, "image2")?;
    let image2 = image2_data.to_string(&mut cx)?.value(&mut cx);
    if image2 == "undefined" {
        return cx.throw_error("'image2' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.fusion(image1, image2) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn gru_plan(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let first_setp_data: Handle<JsValue> = data.get(&mut cx, "first_setp")?;
    let first_setp = first_setp_data.to_string(&mut cx)?.value(&mut cx);
    if first_setp == "undefined" {
        return cx.throw_error("'first_setp' field is missing");
    }
    let second_step_data: Handle<JsValue> = data.get(&mut cx, "second_step")?;
    let second_step = second_step_data.to_string(&mut cx)?.value(&mut cx);
    if second_step == "undefined" {
        return cx.throw_error("'second_step' field is missing");
    }
    let third_step_data: Handle<JsValue> = data.get(&mut cx, "third_step")?;
    let third_step = third_step_data.to_string(&mut cx)?.value(&mut cx);
    if third_step == "undefined" {
        return cx.throw_error("'third_step' field is missing");
    }
    let fourth_step_data: Handle<JsValue> = data.get(&mut cx, "fourth_step")?;
    let fourth_step = fourth_step_data.to_string(&mut cx)?.value(&mut cx);
    if fourth_step == "undefined" {
        return cx.throw_error("'fourth_step' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.gru_plan(first_setp, second_step, third_step, fourth_step) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn heartbreaking(
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
    match st.heartbreaking(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn hitler(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.hitler(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn jail(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.jail(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn jokeoverhead(
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
    match st.jokeoverhead(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn karaba(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.karaba(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn kiss(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image1_data: Handle<JsValue> = data.get(&mut cx, "image1")?;
    let image1 = image1_data.to_string(&mut cx)?.value(&mut cx);
    if image1 == "undefined" {
        return cx.throw_error("'image1' field is missing");
    }
    let image2_data: Handle<JsValue> = data.get(&mut cx, "image2")?;
    let image2 = image2_data.to_string(&mut cx)?.value(&mut cx);
    if image2 == "undefined" {
        return cx.throw_error("'image2' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.kiss(image1, image2) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn kyon_gun(
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
    match st.kyon_gun(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn lisa_presentation(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let text_data: Handle<JsValue> = data.get(&mut cx, "text")?;
    let text = text_data.to_string(&mut cx)?.value(&mut cx);
    if text == "undefined" {
        return cx.throw_error("'text' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.lisa_presentation(text) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn mikkelsen(
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
    match st.mikkelsen(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn mirror(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.mirror(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn mms(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.mms(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn notstonk(
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
    match st.notstonk(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn ohno(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let text_data: Handle<JsValue> = data.get(&mut cx, "text")?;
    let text = text_data.to_string(&mut cx)?.value(&mut cx);
    if text == "undefined" {
        return cx.throw_error("'text' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.ohno(text) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn plankton_plan(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let first_setp_data: Handle<JsValue> = data.get(&mut cx, "first_setp")?;
    let first_setp = first_setp_data.to_string(&mut cx)?.value(&mut cx);
    if first_setp == "undefined" {
        return cx.throw_error("'first_setp' field is missing");
    }
    let second_step_data: Handle<JsValue> = data.get(&mut cx, "second_step")?;
    let second_step = second_step_data.to_string(&mut cx)?.value(&mut cx);
    if second_step == "undefined" {
        return cx.throw_error("'second_step' field is missing");
    }
    let third_step_data: Handle<JsValue> = data.get(&mut cx, "third_step")?;
    let third_step = third_step_data.to_string(&mut cx)?.value(&mut cx);
    if third_step == "undefined" {
        return cx.throw_error("'third_step' field is missing");
    }
    let fourth_step_data: Handle<JsValue> = data.get(&mut cx, "fourth_step")?;
    let fourth_step = fourth_step_data.to_string(&mut cx)?.value(&mut cx);
    if fourth_step == "undefined" {
        return cx.throw_error("'fourth_step' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.plankton_plan(first_setp, second_step, third_step, fourth_step) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn poutine(
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
    match st.poutine(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn rip(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.rip(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn shit(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.shit(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn snyder(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.snyder(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn spank(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image1_data: Handle<JsValue> = data.get(&mut cx, "image1")?;
    let image1 = image1_data.to_string(&mut cx)?.value(&mut cx);
    if image1 == "undefined" {
        return cx.throw_error("'image1' field is missing");
    }
    let image2_data: Handle<JsValue> = data.get(&mut cx, "image2")?;
    let image2 = image2_data.to_string(&mut cx)?.value(&mut cx);
    if image2 == "undefined" {
        return cx.throw_error("'image2' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.spank(image1, image2) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn stonk(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.stonk(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn tattoo(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.tattoo(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn thomas(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.thomas(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn trash(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.trash(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn wanted(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.wanted(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn worthless(
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
    match st.worthless(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn youtube(
    mut cx: FunctionContext,
    strange: Arc<Mutex<StrangeApi>>,
) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let username_data: Handle<JsValue> = data.get(&mut cx, "username")?;
    let username = username_data.to_string(&mut cx)?.value(&mut cx);
    if username == "undefined" {
        return cx.throw_error("'username' field is missing");
    }
    let text_data: Handle<JsValue> = data.get(&mut cx, "text")?;
    let text = text_data.to_string(&mut cx)?.value(&mut cx);
    if text == "undefined" {
        return cx.throw_error("'text' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.youtube(image, username, text) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}
