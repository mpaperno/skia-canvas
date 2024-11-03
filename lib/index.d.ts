/// <reference lib="dom"/>
/// <reference types="node" />

export class DOMPoint extends globalThis.DOMPoint {}
export class DOMRect extends globalThis.DOMRect {}
export class CanvasGradient extends globalThis.CanvasGradient {}
export class CanvasTexture {}

//
// Images
//

export type ColorType =
  /** alias for BGRA8888 */
  "rgba"               |
  /** alias for RGB888x */
  "rgb"                |
  /** alias for BGRA8888 */
  "bgra"               |
  /** alias for ARGB4444 */
  "argb"               |
  // these correspond to skia_safe::ColorType enum names and are mapped in src/utils.rs
  "Alpha8"             |
  "RGB565"             |
  "ARGB4444"           |
  "RGBA8888"           |
  "RGB888x"            |
  "BGRA8888"           |
  "RGBA1010102"        |
  "BGRA1010102"        |
  "RGB101010x"         |
  "BGR101010x"         |
  "Gray8"              |
  "RGBAF16Norm"        |
  "RGBAF16"            |
  "RGBAF32"            |
  "R8G8UNorm"          |
  "A16Float"           |
  "R16G16Float"        |
  "A16UNorm"           |
  "R16G16UNorm"        |
  "R16G16B16A16UNorm"  |
  "SRGBA8888"          |
  "R8UNorm"            |
  /** N32 indicates to use the default color type of the paint device,
   * which must be an actual surface (not raw pixels). */
  "N32"
;

export interface ImageInfo {
  /** Image width */
  width: number
  /** Image height */
  height: number
  /** Color type of pixel. Default is 'rgba'. */
  colorType?: ColorType,
  /** Whether image color data is premultiplied with alpha value. Default is `false`. */
  premultiplied?: boolean,
}

/** Options for `loadImage` and `Image()` constructor. */
export interface ImageOptions {
  /** Image width */
  width?: number
  /** Image height */
  height?: number
  /** Signals to the parser that source is in SVG format. Used when loading image source data from a buffer. */
  type?: 'svg' | undefined
  /** Describes how to process raw image buffer with decoded pixels */
  raw?: ImageInfo | undefined
}

export class Image {
  constructor(width?: number, height?: number)
  constructor(options: ImageOptions)
  get src(): string
  set src(src: string | Buffer)
  get width(): number
  set width(w:number)
  get height(): number
  set height(h:number)
  get naturalWidth(): number
  get naturalHeight(): number
  get complete(): boolean
  decode(): Promise<Image>
  onload: ((this: Image, image: Image) => any) | null
  onerror: ((this: Image, error: Error) => any) | null
}

export function loadImage(src: string | Buffer, options?: ImageOptions): Promise<Image>

/** Extended ImageDataSettings for the extended ImageData type. */
export interface ImageDataSettings extends globalThis.ImageDataSettings {
  /** Color type of pixel. Defaults to 'rgba'. */
  colorType?: ColorType
  /** Whether stored color data is pre-multiplied with alpha value. Default is `undefined` (unknown). */
  premultiplied?: boolean
}

/** An extension of the standard ImageData type. */
export class ImageData extends globalThis.ImageData {
  constructor(sw: number, sh: number, settings?: ImageDataSettings);
  constructor(data: Uint8ClampedArray | Buffer, sw: number, sh?: number, settings?: ImageDataSettings);
  constructor(other: ImageData);
  /** Color type of pixel data. Typically 'rgba' ('RGBA8888') unless specifically set when ImageData was created. */
  readonly colorType: ColorType
  /** Number of bytes representing one pixel in the data. This will depend on the `colorType` property. */
  readonly bytesPerPixel: number
  /** Whether stored color data is pre-multiplied with alpha value. `undefined` if unknown (ie. was not specified). */
  readonly premultiplied: boolean | undefined
}

/** Utility function to look up depth of a given color type. */
export function colorTypeBytesPerPixel(colorType: ColorType): number


//
// DOMMatrix
//

type FixedLenArray<T, L extends number> = T[] & { length: L };
type Matrix = DOMMatrix | { a: number, b: number, c: number, d: number, e: number, f: number } | FixedLenArray<number, 6> | FixedLenArray<number, 16>

export class DOMMatrix extends globalThis.DOMMatrix {
  constructor(matrix: Matrix)
  clone(): DOMMatrix
}

//
// Canvas
//

export type ExportFormat = "png" | "jpg" | "jpeg" | "pdf" | "svg" | "raw";

export interface RenderOptions {
  /** Page to export: Defaults to 1 (i.e., first page) */
  page?: number

  /** Background color to draw beneath transparent parts of the canvas */
  matte?: string

  /** Number of pixels per grid ‘point’ (defaults to 1) */
  density?: number

  /** Quality for lossy encodings like JPEG (0.0–1.0) */
  quality?: number

  /** Convert text to paths for SVG exports */
  outline?: boolean

   /** Render area bounds left origin. Default is (0). */
  left?: number,

  /** Render area bounds top origin. Default is (0). */
  top?:number,

  /** Render area bounds width. Default is canvas width. */
  width?: number

  /** Render area bounds height. Default is canvas height. */
  height?: number

  /** Color type of pixel for raw output. Default is 'rgba'. */
  colorType?: ColorType,

  /** Whether raw output color data is premultiplied with alpha value. Default is `false`. */
  premultiplied?: boolean,
}

export interface SaveOptions extends RenderOptions {
  /** Image format to use */
  format?: ExportFormat
}

export class Canvas {
  /** @internal */
  constructor(width?: number, height?: number)
  static contexts: WeakMap<Canvas, readonly CanvasRenderingContext2D[]>

  /**
   * @deprecated Use the saveAsSync, toBufferSync, and toDataURLSync methods
   * instead of setting the async property to false
   */
  async: boolean
  width: number
  height: number
  size: { width: number, height: number }

  getContext(type?: "2d"): CanvasRenderingContext2D
  newPage(width?: number, height?: number): CanvasRenderingContext2D
  readonly pages: CanvasRenderingContext2D[]

  get gpu(): boolean
  set gpu(enabled: boolean)

  saveAs(filename: string, options?: SaveOptions): Promise<void>
  toBuffer(format: ExportFormat, options?: RenderOptions): Promise<Buffer>
  toDataURL(format: ExportFormat, options?: RenderOptions): Promise<string>
  toRaw(options?: RenderOptions): Promise<Buffer>
  toImageData(options?: RenderOptions): Promise<ImageData>

  saveAsSync(filename: string, options?: SaveOptions): void
  toBufferSync(format: ExportFormat, options?: RenderOptions): Buffer
  toDataURLSync(format: ExportFormat, options?: RenderOptions): string

  get pdf(): Promise<Buffer>
  get svg(): Promise<Buffer>
  get jpg(): Promise<Buffer>
  get png(): Promise<Buffer>
  get webp(): Promise<Buffer>
  get raw(): Promise<Buffer>
}

//
// CanvasPattern
//

export class CanvasPattern {
  setTransform(transform: Matrix): void;
  setTransform(a: number, b: number, c: number, d: number, e: number, f: number): void
}

//
// Context
//

type Offset = [x: number, y: number] | number

export interface CreateTextureOptions {
  /** The 2D shape to be drawn in a repeating grid with the specified spacing (if omitted, parallel lines will be used) */
  path?: Path2D

  /** The lineWidth with which to stroke the path (if omitted, the path will be filled instead) */
  line?: number

  /** The color to use for stroking/filling the path */
  color?: string

  /** The orientation of the pattern grid in radians */
  angle?: number

  /** The amount by which to shift the pattern relative to the canvas origin */
  offset?: Offset
}

export type CanvasImageSource = Canvas | Image;

interface CanvasDrawImage {
  drawImage(image: CanvasImageSource, dx: number, dy: number): void;
  drawImage(image: CanvasImageSource, dx: number, dy: number, dw: number, dh: number): void;
  drawImage(image: CanvasImageSource, sx: number, sy: number, sw: number, sh: number, dx: number, dy: number, dw: number, dh: number): void;
  drawCanvas(image: Canvas, dx: number, dy: number): void;
  drawCanvas(image: Canvas, dx: number, dy: number, dw: number, dh: number): void;
  drawCanvas(image: Canvas, sx: number, sy: number, sw: number, sh: number, dx: number, dy: number, dw: number, dh: number): void;
}

interface CanvasFillStrokeStyles {
  fillStyle: string | CanvasGradient | CanvasPattern | CanvasTexture;
  strokeStyle: string | CanvasGradient | CanvasPattern | CanvasTexture;
  createConicGradient(startAngle: number, x: number, y: number): CanvasGradient;
  createLinearGradient(x0: number, y0: number, x1: number, y1: number): CanvasGradient;
  createRadialGradient(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number): CanvasGradient;
  createPattern(image: CanvasImageSource, repetition: string | null): CanvasPattern | null;
  createTexture(spacing: Offset, options?: CreateTextureOptions): CanvasTexture
}

type QuadOrRect = [x1:number, y1:number, x2:number, y2:number, x3:number, y3:number, x4:number, y4:number] |
                  [left:number, top:number, right:number, bottom:number] | [width:number, height:number]

type CornerRadius = number | DOMPoint

interface CanvasTransform extends Omit<globalThis.CanvasTransform, "transform" | "setTransform">{}

interface CanvasTextDrawingStyles extends Omit<globalThis.CanvasTextDrawingStyles, "fontKerning" | "fontVariantCaps" | "textRendering">{}

export interface CanvasRenderingContext2D extends CanvasCompositing, CanvasDrawImage, CanvasDrawPath, CanvasFillStrokeStyles, CanvasFilters, CanvasImageData, CanvasImageSmoothing, CanvasPath, CanvasPathDrawingStyles, CanvasRect, CanvasShadowStyles, CanvasState, CanvasText, CanvasTextDrawingStyles, CanvasTransform, CanvasUserInterface {
  readonly canvas: Canvas;
  fontVariant: string;
  textTracking: number;
  textWrap: boolean;
  lineDashMarker: Path2D | null;
  lineDashFit: "move" | "turn" | "follow";

  setTransform(transform?: Matrix): void
  setTransform(a: number, b: number, c: number, d: number, e: number, f: number): void

  transform(transform?: Matrix): void
  transform(a: number, b: number, c: number, d: number, e: number, f: number): void

  get currentTransform(): DOMMatrix
  set currentTransform(matrix: Matrix)
  createProjection(quad: QuadOrRect, basis?: QuadOrRect): DOMMatrix

  conicCurveTo(cpx: number, cpy: number, x: number, y: number, weight: number): void
  roundRect(x: number, y: number, width: number, height: number, radii: number | CornerRadius[]): void
  // getContextAttributes(): CanvasRenderingContext2DSettings;

  fillText(text: string, x: number, y:number, maxWidth?: number): void
  strokeText(text: string, x: number, y:number, maxWidth?: number): void
  measureText(text: string, maxWidth?: number): TextMetrics
  outlineText(text: string): Path2D

  reset(): void
}

//
// Bézier Paths
//

export interface Path2DBounds {
  readonly top: number
  readonly left: number
  readonly bottom: number
  readonly right: number
  readonly width: number
  readonly height: number
}

export type Path2DEdge = [verb: string, ...args: number[]]

export class Path2D extends globalThis.Path2D {
  d: string
  readonly bounds: Path2DBounds
  readonly edges: readonly Path2DEdge[]

  contains(x: number, y: number): boolean
  conicCurveTo(
    cpx: number,
    cpy: number,
    x: number,
    y: number,
    weight: number
  ): void

  roundRect(x: number, y: number, width: number, height: number, radii: number | CornerRadius[]): void

  complement(otherPath: Path2D): Path2D
  difference(otherPath: Path2D): Path2D
  intersect(otherPath: Path2D): Path2D
  union(otherPath: Path2D): Path2D
  xor(otherPath: Path2D): Path2D
  interpolate(otherPath: Path2D, weight: number): Path2D

  jitter(segmentLength: number, amount: number, seed?: number): Path2D
  offset(dx: number, dy: number): Path2D
  points(step?: number): readonly [x: number, y: number][]
  round(radius: number): Path2D
  simplify(rule?: "nonzero" | "evenodd"): Path2D
  transform(transform: Matrix): Path2D;
  transform(a: number, b: number, c: number, d: number, e: number, f: number): Path2D;
  trim(start: number, end: number, inverted?: boolean): Path2D;
  trim(start: number, inverted?: boolean): Path2D;

  unwind(): Path2D
}

//
// Typography
//

export interface TextMetrics extends globalThis.TextMetrics {
  lines: TextMetricsLine[]
}

export interface TextMetricsLine {
  readonly x: number
  readonly y: number
  readonly width: number
  readonly height: number
  readonly baseline: number
  readonly startIndex: number
  readonly endIndex: number
}

export interface FontFamily {
  family: string
  weights: number[]
  widths: string[]
  styles: string[]
}

export interface Font {
  family: string
  weight: number
  style: string
  width: string
  file: string
}

export interface FontLibrary {
  families: readonly string[]
  family(name: string): FontFamily | undefined
  has(familyName: string): boolean

  use(familyName: string, fontPaths?: string | readonly string[]): Font[]
  use(fontPaths: readonly string[]): Font[]
  use(
    families: Record<string, readonly string[] | string>
  ): Record<string, Font[] | Font>

  reset(): void
}

export const FontLibrary: FontLibrary

//
// Window & App
//

import { EventEmitter } from "stream";
export type FitStyle = "none" | "contain-x" | "contain-y" | "contain" | "cover" | "fill" | "scale-down" | "resize"
export type CursorStyle = "default" | "crosshair" | "hand" | "arrow" | "move" | "text" | "wait" | "help" | "progress" | "not-allowed" | "context-menu" |
                          "cell" | "vertical-text" | "alias" | "copy" | "no-drop" | "grab" | "grabbing" | "all-scroll" | "zoom-in" | "zoom-out" |
                          "e-resize" | "n-resize" | "ne-resize" | "nw-resize" | "s-resize" | "se-resize" | "sw-resize" | "w-resize" | "ew-resize" |
                          "ns-resize" | "nesw-resize" | "nwse-resize" | "col-resize" | "row-resize" | "none"

export type WindowOptions = {
  title?: string
  left?: number
  top?: number
  width?: number
  height?: number
  fit?: FitStyle
  page?: number
  background?: string
  fullscreen?: boolean
  visible?: boolean
  cursor?: CursorStyle
  canvas?: Canvas
}

export class Window extends EventEmitter{
  constructor(width: number, height: number, options?: WindowOptions)
  constructor(options?: WindowOptions)

  readonly ctx: CanvasRenderingContext2D
  canvas: Canvas
  visible: boolean
  fullscreen: boolean
  title: string
  cursor: CursorStyle
  fit: FitStyle
  left: number
  top: number
  width: number
  height: number
  page: number
  background: string

  close(): void
}

export interface App{
  readonly windows: Window[]
  readonly running: boolean
  fps: number

  launch(): void
  quit(): void
}

export const App: App
