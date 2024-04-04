use neon::prelude::*;
use neon::types::buffer::TypedArray;
use std::sync::{Arc, Mutex};

use crate::StrangeApi;

pub fn blur(mut cx: FunctionContext, strange: Arc<Mutex<StrangeApi>>) -> JsResult<JsArrayBuffer> {
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let amount = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let amount = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let pixels = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
    let image = cx.argument::<JsString>(0)?.value(&mut cx);
    let amount = cx.argument::<JsNumber>(1)?.value(&mut cx);
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
