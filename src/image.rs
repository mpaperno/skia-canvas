#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::cell::RefCell;
use neon::{prelude::*, types::buffer::TypedArray};
use skia_safe::{Image as SkImage, ImageInfo, ISize, ColorType, AlphaType, Data};
use skia_safe::image::images;

use crate::utils::*;


pub type BoxedImage = JsBox<RefCell<Image>>;
impl Finalize for Image {}

pub struct Image{
  src:String,
  size:ISize,
  pub image:Option<SkImage>,
}

impl Image{
  pub fn info(width:f32, height:f32) -> ImageInfo {
    let dims = (width as i32, height as i32);
    ImageInfo::new(dims, ColorType::RGBA8888, AlphaType::Unpremul, None)
  }

  pub fn image_size(&self) -> ISize {
    if let Some(img) = &self.image {
      img.dimensions()
    } else {
      ISize::new_empty()
    }
  }

  pub fn size(&self) -> ISize {
    let mut size = self.size.clone();
    let img_size = self.image_size();
    if size.width < 0 {
      size.width = img_size.width;
    }
    if size.height < 0 {
      size.height = img_size.height;
    }
    size
  }

}

//
// -- Javascript Methods --------------------------------------------------------------------------
//

pub fn new(mut cx: FunctionContext) -> JsResult<BoxedImage> {
  let this = RefCell::new(Image{
    src:"".to_string(),
    size:ISize::new(-1,-1),
    image:None,
  });
  Ok(cx.boxed(this))
}

pub fn get_src(mut cx: FunctionContext) -> JsResult<JsString> {
  let this = cx.argument::<BoxedImage>(0)?;
  let this = this.borrow();

  Ok(cx.string(&this.src))
}

pub fn set_src(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedImage>(0)?;
  let mut this = this.borrow_mut();

  let src = cx.argument::<JsString>(1)?.value(&mut cx);
  this.src = src;
  Ok(cx.undefined())
}

pub fn set_data(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let this = cx.argument::<BoxedImage>(0)?;
  let mut this = this.borrow_mut();

  let buffer = cx.argument::<JsBuffer>(1)?;
  let data = Data::new_copy(buffer.as_slice(&mut cx));

  this.image = images::deferred_from_encoded_data(data, None);
  Ok(cx.boolean(this.image.is_some()))
}

pub fn load_pixel_data(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let this = cx.argument::<BoxedImage>(0)?;
  let mut this = this.borrow_mut();

  let buffer = cx.argument::<JsBuffer>(1)?;
  let data = Data::new_copy(buffer.as_slice(&mut cx));

  let image_parameters = cx.argument::<JsObject>(2)?;
  let js_width: Handle<JsNumber> = image_parameters.get(&mut cx, "width")?;
  let js_height: Handle<JsNumber> = image_parameters.get(&mut cx, "height")?;
  let js_color_type: Option<Handle<JsString>> = image_parameters.get_opt(&mut cx, "colorType")?;
  let js_premult: Option<Handle<JsBoolean>> = image_parameters.get_opt(&mut cx, "premultiplied")?;

  let width = js_width.value(&mut cx) as i32;
  let height = js_height.value(&mut cx) as i32;
  let ctype = if js_color_type.is_some() { Some(to_color_type(js_color_type.unwrap().value(&mut cx).as_str())) } else { None };
  let premult = if js_premult.is_some() { Some(js_premult.unwrap().value(&mut cx)) } else { Some(false) };

  let image_info = make_raw_image_info((width, height), premult, ctype);
  this.image = images::raster_from_data(&image_info, data, image_info.min_row_bytes());

  Ok(cx.boolean(this.image.is_some()))
}

pub fn get_width(mut cx: FunctionContext) -> JsResult<JsValue> {
  let this = cx.argument::<BoxedImage>(0)?;
  let this = this.borrow();
  Ok(cx.number(this.size().width).upcast())
}

pub fn set_width(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedImage>(0)?;
  let mut this = this.borrow_mut();
  if let Some(num) = opt_float_arg(&mut cx, 1){
    this.size.width = i32::max(0, num as i32);
  }
  Ok(cx.undefined())
}

pub fn get_height(mut cx: FunctionContext) -> JsResult<JsValue> {
  let this = cx.argument::<BoxedImage>(0)?;
  let this = this.borrow();
  Ok(cx.number(this.size().height).upcast())
}

pub fn set_height(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedImage>(0)?;
  let mut this = this.borrow_mut();
  if let Some(num) = opt_float_arg(&mut cx, 1){
    this.size.height = i32::max(0, num as i32);
  }
  Ok(cx.undefined())
}

pub fn get_natural_width(mut cx: FunctionContext) -> JsResult<JsValue> {
  let this = cx.argument::<BoxedImage>(0)?;
  let this = this.borrow();
  Ok(cx.number(this.image_size().width).upcast())
}

pub fn get_natural_height(mut cx: FunctionContext) -> JsResult<JsValue> {
  let this = cx.argument::<BoxedImage>(0)?;
  let this = this.borrow();
  Ok(cx.number(this.image_size().height).upcast())
}

pub fn get_complete(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let this = cx.argument::<BoxedImage>(0)?;
  let this = this.borrow();
  Ok(cx.boolean(this.image.is_some()))
}
