use num_traits::FromPrimitive;

#[repr(C)]
#[derive(num_derive::FromPrimitive)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SdlEventType {
    SdlNoEvent = 0,
    SdlActiveEvent,
    SdlKeyDown,
    SdlKeyUp,
    SdlMouseMotion,
    SdlMouseButtonDown,
    SdlMouseButtonUp,
    SdlJoyAxisMotion,
    SdlJoyBallMotion,
    SdlJoyHatMotion,
    SdlJoyButtonDown,
    SdlJoyButtonUp,
    SdlQuit,
    SdlSysWMEvent,
    SdlEventReservedA,
    SdlEventReservedB,
    SdlVideoResize,
    SdlVideoExpose,
    SdlEventReserved2,
    SdlEventReserved3,
    SdlEventReserved4,
    SdlEventReserved5,
    SdlEventReserved6,
    SdlEventReserved7,
    SdlUserEvent = 24,
    SdlNumEvents = 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SdlKey {
    SdlkUnknown = 0,
    SdlkBackspace = 8,
    SdlkTab = 9,
    SdlkReturn = 13,
    SdlkEscape = 27,
    SdlkSpace = 32,
    Sdlk0 = 48,
    Sdlk1 = 49,
    Sdlk2 = 50,
    Sdlk3 = 51,
    Sdlk4 = 52,
    Sdlk5 = 53,
    Sdlk6 = 54,
    Sdlk7 = 55,
    Sdlk8 = 56,
    Sdlk9 = 57,
    SdlkA = 97,
    SdlkD = 100,
    SdlkE = 101,
    SdlkQ = 113,
    SdlkS = 115,
    SdlkT = 116,
    SdlkW = 119,
    SdlkDelete = 127,
    SdlkUp = 273,
    SdlkDown = 274,
    SdlkRight = 275,
    SdlkLeft = 276,
    SdlkF1 = 282,
    SdlkF2 = 283,
    SdlkF5 = 286,
    SdlkF11 = 292,
    SdlkF12 = 293,
    SdlkRshift = 303,
    SdlkLshift = 304,
    SdlkWorld0 = 160 // Used For Controller Crafting Button
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SdlMod {
    KmodNone = 0x0,
    KmodLshift = 0x1,
    KmodRshift = 0x2,
    KmodLctrl = 0x40,
    KmodRctrl = 0x80,
    KmodLalt = 0x100,
    KmodRalt = 0x200
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlActiveEvent {
    pub ev_type: u8,
    pub gain: u8,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlKeysym {
    pub scancode: u8,
    pub sym: SdlKey,
    pub modifier: SdlMod,
    pub unicode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlKeyboardEvent {
    pub ev_type: u8,
    pub which: u8,
    pub state: u8,
    pub keysym: SdlKeysym,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlMouseMotionEvent {
    pub ev_type: u8,
    pub which: u8,
    pub state: u8,
    pub x: u16,
    pub y: u16,
    pub xrel: i16,
    pub yrel: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlMouseButtonEvent {
    pub ev_type: u8,
    pub which: u8,
    pub button: u8,
    pub state: u8,
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlJoyAxisEvent {
    pub ev_type: u8,
    pub which: u8,
    pub axis: u8,
    pub value: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlJoyBallEvent {
    pub  ev_type: u8,
    pub which: u8,
    pub ball: u8,
    pub xrel: i16,
    pub yrel: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlJoyHatEvent {
    pub ev_type: u8,
    pub which: u8,
    pub hat: u8,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlJoyButtonEvent {
    pub ev_type: u8,
    pub which: u8,
    pub button: u8,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlResizeEvent {
    pub ev_type: u8,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlExposeEvent {
    pub ev_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlQuitEvent {
    pub ev_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlUserEvent {
    pub ev_type: u8,
    pub code: i32,
    pub data1: u32,
    pub data2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SdlSysWMEvent {
    pub ev_type: u8,
    pub msg: u32,
}

#[repr(C)]
pub union SdlEvent {
    pub ev_type: u8,
    pub active: SdlActiveEvent,
    pub key: SdlKeyboardEvent,
    pub motion: SdlMouseMotionEvent,
    pub button: SdlMouseButtonEvent,
    pub jaxis: SdlJoyAxisEvent,
    pub jball: SdlJoyBallEvent,
    pub jhat: SdlJoyHatEvent,
    pub jbutton: SdlJoyButtonEvent,
    pub resize: SdlResizeEvent,
    pub expose: SdlExposeEvent,
    pub quit: SdlQuitEvent,
    pub user: SdlUserEvent,
    pub syswm: SdlSysWMEvent,
}

impl SdlEvent {
    pub fn event_type(&self) -> Option<SdlEventType> {
        <SdlEventType as FromPrimitive>::from_u8(unsafe { self.ev_type })
    }
}

extern "C" {
    pub fn SDL_PushEvent(ev: *const SdlEvent);
}