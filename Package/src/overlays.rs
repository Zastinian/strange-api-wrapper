use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

use crate::StrangeApi;

pub fn approved(
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
    match st.approved(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn brazzers(
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
    match st.brazzers(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn gay(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.gay(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn halloween(
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
    match st.halloween(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn rejected(
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
    match st.rejected(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn thuglife(
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
    match st.thuglife(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn to_be_continued(
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
    match st.to_be_continued(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}

pub fn wasted(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsObject>(0)?;
    let image_data: Handle<JsValue> = data.get(&mut cx, "image")?;
    let image = image_data.to_string(&mut cx)?.value(&mut cx);
    if image == "undefined" {
        return cx.throw_error("'image' field is missing");
    }
    let st = strange.lock().unwrap();
    match st.wasted(image) {
        Ok(buffer) => {
            let mut js_buffer = cx.array_buffer(buffer.len())?;
            js_buffer.as_mut_slice(&mut cx).copy_from_slice(&buffer);
            Ok(js_buffer)
        }
        Err(err) => cx.throw_error(format!("Failed: {}", err)),
    }
}
