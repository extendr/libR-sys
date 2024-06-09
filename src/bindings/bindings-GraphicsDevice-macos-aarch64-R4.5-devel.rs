/* automatically generated by rust-bindgen 0.69.4 */

/* libR-sys version: 0.7.0 */
/* bindgen clang version: Homebrew clang version 18.1.6 */
/* r version: 4.5.0-devel */

pub const R_USE_PROTOTYPES: u32 = 1;
pub const leftButton: u32 = 1;
pub const middleButton: u32 = 2;
pub const rightButton: u32 = 4;
#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[doc = "--------- New (in 1.4.0) device driver structure ---------\n NOTES:\n 1. All locations and dimensions are in device coordinates.\n 2. I found this comment in the doc for dev_Open -- looks nasty\n    Any known instances of such a thing happening?  Should be\n    replaced by a function to query the device for preferred gpars\n    settings? (to be called when the device is initialised)\n\n NOTE that it is perfectly acceptable for this\n function to set generic graphics parameters too\n (i.e., override the generic parameter settings\n which GInit sets up) all at the author's own risk\n of course :)\n\n 3. Do we really need dev_StrWidth as well as dev_MetricInfo?\n    I can see the difference between the two -- its just a\n    question of whether dev_MetricInfo should just return\n    what dev_StrWidth would give if font metric information is\n    not available.  I guess having both allows the developer\n    to decide when to ask for which sort of value, and to decide\n    what to do when font metric information is not available.\n    And why not a dev_StrHeight?\n 4. Should \"ipr\", \"asp\", and \"cra\" be in the device description?\n    If not, then where?\n    I guess they don't need to be if no device makes use of them.\n    On the other hand, they would need to be replaced by a device\n    call that R base graphics could use to get enough information\n    to figure them out.  (e.g., some sort of dpi() function to\n    complement the size() function.)"]
pub type DevDesc = _DevDesc;
pub type pDevDesc = *mut DevDesc;
#[repr(C)]
pub struct _DevDesc {
    #[doc = "left raster coordinate"]
    pub left: f64,
    #[doc = "right raster coordinate"]
    pub right: f64,
    #[doc = "bottom raster coordinate"]
    pub bottom: f64,
    #[doc = "top raster coordinate"]
    pub top: f64,
    #[doc = "R only has the notion of a rectangular clipping region"]
    pub clipLeft: f64,
    pub clipRight: f64,
    pub clipBottom: f64,
    pub clipTop: f64,
    #[doc = "x character addressing offset - unused"]
    pub xCharOffset: f64,
    #[doc = "y character addressing offset"]
    pub yCharOffset: f64,
    #[doc = "1/2 interline space as frac of line height"]
    pub yLineBias: f64,
    #[doc = "Inches per raster; \\[0\\]=x, \\[1\\]=y"]
    pub ipr: [f64; 2usize],
    #[doc = "Character size in rasters; \\[0\\]=x, \\[1\\]=y"]
    pub cra: [f64; 2usize],
    #[doc = "(initial) Device Gamma Correction"]
    pub gamma: f64,
    #[doc = "Device-level clipping"]
    pub canClip: Rboolean,
    #[doc = "can the gamma factor be modified?"]
    pub canChangeGamma: Rboolean,
    #[doc = "Can do at least some horiz adjust of text\n0 = none, 1 = {0,0.5,1}, 2 = \\[0,1\\]"]
    pub canHAdj: ::std::os::raw::c_int,
    #[doc = "Device initial settings\n/\n/* These are things that the device must set up when it is created.\n The graphics system can modify them and track current values,"]
    pub startps: f64,
    #[doc = "sets par(\"fg\"), par(\"col\") and gpar(\"col\")"]
    pub startcol: ::std::os::raw::c_int,
    #[doc = "sets par(\"bg\") and gpar(\"fill\")"]
    pub startfill: ::std::os::raw::c_int,
    pub startlty: ::std::os::raw::c_int,
    pub startfont: ::std::os::raw::c_int,
    pub startgamma: f64,
    #[doc = "pointer to device specific parameters"]
    pub deviceSpecific: *mut ::std::os::raw::c_void,
    #[doc = "toggle for initial display list status"]
    pub displayListOn: Rboolean,
    #[doc = "can the device generate mousedown events"]
    pub canGenMouseDown: Rboolean,
    #[doc = "can the device generate mousemove events"]
    pub canGenMouseMove: Rboolean,
    #[doc = "can the device generate mouseup events"]
    pub canGenMouseUp: Rboolean,
    #[doc = "can the device generate keyboard events"]
    pub canGenKeybd: Rboolean,
    #[doc = "can the device generate idle events"]
    pub canGenIdle: Rboolean,
    #[doc = "This is set while getGraphicsEvent\nis actively looking for events"]
    pub gettingEvent: Rboolean,
    pub activate: ::std::option::Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    pub circle: ::std::option::Option<
        unsafe extern "C" fn(x: f64, y: f64, r: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub clip: ::std::option::Option<
        unsafe extern "C" fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: pDevDesc),
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc)>,
    pub deactivate: ::std::option::Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    pub locator: ::std::option::Option<
        unsafe extern "C" fn(x: *mut f64, y: *mut f64, dd: pDevDesc) -> Rboolean,
    >,
    pub line: ::std::option::Option<
        unsafe extern "C" fn(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub metricInfo: ::std::option::Option<
        unsafe extern "C" fn(
            c: ::std::os::raw::c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ),
    >,
    pub mode:
        ::std::option::Option<unsafe extern "C" fn(mode: ::std::os::raw::c_int, dd: pDevDesc)>,
    pub newPage: ::std::option::Option<unsafe extern "C" fn(gc: pGEcontext, dd: pDevDesc)>,
    pub polygon: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub polyline: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub rect: ::std::option::Option<
        unsafe extern "C" fn(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub path: ::std::option::Option<
        unsafe extern "C" fn(
            x: *mut f64,
            y: *mut f64,
            npoly: ::std::os::raw::c_int,
            nper: *mut ::std::os::raw::c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub raster: ::std::option::Option<
        unsafe extern "C" fn(
            raster: *mut ::std::os::raw::c_uint,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            x: f64,
            y: f64,
            width: f64,
            height: f64,
            rot: f64,
            interpolate: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub cap: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc) -> SEXP>,
    pub size: ::std::option::Option<
        unsafe extern "C" fn(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ),
    >,
    pub strWidth: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64,
    >,
    pub text: ::std::option::Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str_: *const ::std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub onExit: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc)>,
    #[doc = "device_getEvent is no longer used, but the slot is kept for back\n compatibility of the structure."]
    pub getEvent: ::std::option::Option<
        unsafe extern "C" fn(arg1: SEXP, arg2: *const ::std::os::raw::c_char) -> SEXP,
    >,
    pub newFrameConfirm: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc) -> Rboolean>,
    #[doc = "and strWidthUTF8"]
    pub hasTextUTF8: Rboolean,
    pub textUTF8: ::std::option::Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str_: *const ::std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub strWidthUTF8: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64,
    >,
    pub wantSymbolUTF8: Rboolean,
    #[doc = "Is rotated text good enough to be preferable to Hershey in\ncontour labels?  Old default was FALSE."]
    pub useRotatedTextInContour: Rboolean,
    #[doc = "This is an environment holding event handlers."]
    pub eventEnv: SEXP,
    pub eventHelper:
        ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc, code: ::std::os::raw::c_int)>,
    pub holdflush: ::std::option::Option<
        unsafe extern "C" fn(dd: pDevDesc, level: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    #[doc = "1 = no, 2 = yes"]
    pub haveTransparency: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = fully, 3 = semi"]
    pub haveTransparentBg: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes, 3 = except for missing values"]
    pub haveRaster: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes"]
    pub haveCapture: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes"]
    pub haveLocator: ::std::os::raw::c_int,
    pub setPattern:
        ::std::option::Option<unsafe extern "C" fn(pattern: SEXP, dd: pDevDesc) -> SEXP>,
    pub releasePattern: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub setClipPath:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,
    pub releaseClipPath: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub setMask:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,
    pub releaseMask: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    #[doc = "This should match R_GE_version,\n BUT it does not have to.\n It give the graphics engine a chance to work with\n graphics device packages BEFORE they update to\n changes in R_GE_version."]
    pub deviceVersion: ::std::os::raw::c_int,
    #[doc = "This can be used to OVERRIDE canClip so that graphics engine\n leaves ALL clipping to the graphics device"]
    pub deviceClip: Rboolean,
    pub defineGroup: ::std::option::Option<
        unsafe extern "C" fn(
            source: SEXP,
            op: ::std::os::raw::c_int,
            destination: SEXP,
            dd: pDevDesc,
        ) -> SEXP,
    >,
    pub useGroup:
        ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, trans: SEXP, dd: pDevDesc)>,
    pub releaseGroup: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub stroke:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, gc: pGEcontext, dd: pDevDesc)>,
    pub fill: ::std::option::Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,
    pub fillStroke: ::std::option::Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,
    pub capabilities: ::std::option::Option<unsafe extern "C" fn(cap: SEXP) -> SEXP>,
    pub glyph: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            glyphs: *mut ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            font: SEXP,
            size: f64,
            colour: ::std::os::raw::c_int,
            rot: f64,
            dd: pDevDesc,
        ),
    >,
    #[doc = "Area for future expansion.\nBy zeroing this, devices are more likely to work if loaded\ninto a later version of R than that they were compiled under."]
    pub reserved: [::std::os::raw::c_char; 64usize],
}
#[repr(i32)]
#[doc = "These give the indices of some known keys"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_KeyName {
    knUNKNOWN = -1,
    knLEFT = 0,
    knUP = 1,
    knRIGHT = 2,
    knDOWN = 3,
    knF1 = 4,
    knF2 = 5,
    knF3 = 6,
    knF4 = 7,
    knF5 = 8,
    knF6 = 9,
    knF7 = 10,
    knF8 = 11,
    knF9 = 12,
    knF10 = 13,
    knF11 = 14,
    knF12 = 15,
    knPGUP = 16,
    knPGDN = 17,
    knEND = 18,
    knHOME = 19,
    knINS = 20,
    knDEL = 21,
}
#[repr(u32)]
#[doc = "These are the three possible mouse events"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_MouseEvent {
    meMouseDown = 0,
    meMouseUp = 1,
    meMouseMove = 2,
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
extern "C" {
    #[doc = "Properly declared version of devNumber"]
    pub fn Rf_ndevNumber(arg1: pDevDesc) -> ::std::os::raw::c_int;
    #[doc = "How many devices exist ? (>= 1)"]
    pub fn Rf_NumDevices() -> ::std::os::raw::c_int;
    #[doc = "Check for an available device slot"]
    pub fn R_CheckDeviceAvailable();
    pub fn R_CheckDeviceAvailableBool() -> Rboolean;
    #[doc = "Return the number of the current device."]
    pub fn Rf_curDevice() -> ::std::os::raw::c_int;
    #[doc = "Return the number of the next device."]
    pub fn Rf_nextDevice(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    #[doc = "Return the number of the previous device."]
    pub fn Rf_prevDevice(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    #[doc = "Make the specified device (specified by number) the current device"]
    pub fn Rf_selectDevice(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    #[doc = "Kill device which is identified by number."]
    pub fn Rf_killDevice(arg1: ::std::os::raw::c_int);
    pub fn Rf_NoDevices() -> ::std::os::raw::c_int;
    pub fn Rf_NewFrameConfirm(arg1: pDevDesc);
    pub fn Rf_doMouseEvent(
        dd: pDevDesc,
        event: R_MouseEvent,
        buttons: ::std::os::raw::c_int,
        x: f64,
        y: f64,
    );
    pub fn Rf_doKeybd(dd: pDevDesc, rkey: R_KeyName, keyname: *const ::std::os::raw::c_char);
    pub fn Rf_doIdle(dd: pDevDesc);
    pub fn Rf_doesIdle(dd: pDevDesc) -> Rboolean;
    pub static mut R_interrupts_suspended: Rboolean;
    pub static mut R_interrupts_pending: ::std::os::raw::c_int;
    pub fn Rf_onintr();
    pub static mut mbcslocale: Rboolean;
    #[doc = "Useful for devices: translates Adobe symbol encoding to UTF-8"]
    pub fn Rf_AdobeSymbol2utf8(
        out: *mut ::std::os::raw::c_char,
        in_: *const ::std::os::raw::c_char,
        nwork: usize,
        usePUA: Rboolean,
    ) -> *mut ::std::os::raw::c_void;
    pub fn Rf_utf8toAdobeSymbol(
        out: *mut ::std::os::raw::c_char,
        in_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_utf8Toutf8NoPUA(in_: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
    pub fn Rf_utf8ToLatin1AdobeSymbol2utf8(
        in_: *const ::std::os::raw::c_char,
        usePUA: Rboolean,
    ) -> *const ::std::os::raw::c_char;
    #[doc = "Translates Unicode point to UTF-8"]
    pub fn Rf_ucstoutf8(s: *mut ::std::os::raw::c_char, c: ::std::os::raw::c_uint) -> usize;
}
