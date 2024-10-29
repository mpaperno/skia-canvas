#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::cell::RefCell;
use neon::{prelude::*, types::buffer::TypedArray};
use skia_safe::{Image as SkImage, ImageInfo, ISize, ColorType, AlphaType, Data,
                FontMgr, Picture, PictureRecorder, Rect};
use skia_safe::image::images;
use skia_safe::svg;
use skia_safe::wrapper::PointerWrapper;  // for SVG Dom access, temporary until next skia-safe update

use crate::utils::*;
use crate::FONT_LIBRARY;


pub type BoxedImage = JsBox<RefCell<Image>>;
impl Finalize for Image {}

pub struct Image{
  src:String,
  size:ISize,
  pub adjust_size_to_canvas:bool,
  pub image:Option<SkImage>,
  pub picture:Option<Picture>
}

impl Image{
  pub fn info(width:f32, height:f32) -> ImageInfo {
    let dims = (width as i32, height as i32);
    ImageInfo::new(dims, ColorType::RGBA8888, AlphaType::Unpremul, None)
  }

  pub fn image_size(&self) -> ISize {
    if let Some(img) = &self.image {
      img.dimensions()
    } else if let Some(pict) = &self.picture {
      pict.cull_rect().size().to_ceil()
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
    adjust_size_to_canvas:false,
    image:None,
    picture:None,
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

pub fn load_svg(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let this = cx.argument::<BoxedImage>(0)?;
  let mut this = this.borrow_mut();

  let buffer = cx.argument::<JsBuffer>(1)?;
  let data = Data::new_copy(buffer.as_slice(&mut cx));

  // We need an instance of a FontMgr for the DOM loader to use when parsing fonts in the SVG.  // TODO: Is this right?
  let mgr = FONT_LIBRARY.lock().unwrap().collection.fallback_manager().unwrap_or(FontMgr::default());
  // Parse & load the SVG data.
  let dom = svg::Dom::from_bytes(&data, mgr);
  if !dom.is_ok() {
    return cx.throw_type_error("Error loading SVG data.")
  }
  let mut dom = dom.unwrap();

  // Get the intrinsic size of the `svg` root element as specified in the width/height attributes, if any.
  // So far skia-safe doesn't provide direct access to the needed methods, so we have to go direct to the source.
  let i_size = unsafe { *dom.inner().containerSize() };  // skia_bindings::SkSize
  // let i_size = dom.inner().fContainerSize;  // "safe" but this is using a private member of the C++ class (somehow... skia-"safe" :-P )
  // TODO: Switch to these once available in skia-safe 0.79+
  // let mut root = dom.root();
  // let i_size = root.intrinsic_size();

  let mut bounds = Rect::from_wh(i_size.fWidth, i_size.fHeight);

  // Set a flag to indicate that the image doesn't have its own intrinsic size.
  // This may be used at drawing time if user doesn't specify a size in `drawImage()`,
  // in which case the the canvas' size will be used as the image size.
  // This is a "complication" to match Chrome's behavior... one could argue that it should
  // just be drawn at the default size (set below). Which is what FF does (though that has its own anomalies).
  this.adjust_size_to_canvas = bounds.is_empty();

  // Check if width/height are valid attribute values in the root `<svg>` element.
  // If w/h aren't specified in an SVG (which is not uncommon), both Chrome and FF will:
  //  - If only one dimension is missing then use the same size for both;
  //  - If both are missing then assign a default of 150 (which seems arbitrary but I guess as good as any);
  // `Dom::containerSize()` will return zero for both width and height if _either_ attribute is missing from `<svg>`.
  // This seems a bit suspicious (as in may change in future?), so in the interest of paranoia let's check them individually.
  // TODO: See if we can get actual width/height attribute values from DOM with skia-safe 0.79+
  if bounds.right <= 0.0 {
    bounds.right = match bounds.bottom > 0.0 {
      true  => bounds.bottom,
      false => 150.0f32
    };
  }
  if bounds.bottom <= 0.0 {
    bounds.bottom = bounds.right;
  }

  // If there is no intrinsic size to the SVG then
  // this will update it with our defaults, otherwise this is a no-op.
  dom.set_container_size(bounds.size());

  // Save the image as a Picture so it can be scaled properly later.
  let mut compositor = PictureRecorder::new();
  compositor.begin_recording(bounds, None);
  if let Some(canvas) = compositor.recording_canvas() {
    dom.render(canvas);
  }
  this.picture = compositor.finish_recording_as_picture(Some(&bounds));

  Ok(cx.boolean(this.picture.is_some()))
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
  Ok(cx.boolean(this.image.is_some() || this.picture.is_some()))
}
