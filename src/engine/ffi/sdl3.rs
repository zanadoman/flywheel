pub mod sdl_error {
    use core::ffi::c_char;

    #[link(name = "SDL3")]
    unsafe extern "C" {
        #[must_use]
        pub fn SDL_GetError() -> *const c_char;
    }
}

pub mod sdl_init {
    use core::ffi::{c_char, c_uint};

    pub const SDL_INIT_AUDIO: c_uint = 0x0000_0010;
    pub const SDL_INIT_VIDEO: c_uint = 0x0000_0020;
    pub const SDL_INIT_EVENTS: c_uint = 0x000_04000;

    pub const SDL_PROP_APP_METADATA_NAME_STRING: *const c_char =
        c"SDL.app.metadata.name".as_ptr();
    pub const SDL_PROP_APP_METADATA_VERSION_STRING: *const c_char =
        c"SDL.app.metadata.version".as_ptr();
    pub const SDL_PROP_APP_METADATA_IDENTIFIER_STRING: *const c_char =
        c"SDL.app.metadata.identifier".as_ptr();
    pub const SDL_PROP_APP_METADATA_CREATOR_STRING: *const c_char =
        c"SDL.app.metadata.creator".as_ptr();
    pub const SDL_PROP_APP_METADATA_COPYRIGHT_STRING: *const c_char =
        c"SDL.app.metadata.copyright".as_ptr();
    pub const SDL_PROP_APP_METADATA_URL_STRING: *const c_char =
        c"SDL.app.metadata.url".as_ptr();
    pub const SDL_PROP_APP_METADATA_TYPE_STRING: *const c_char =
        c"SDL.app.metadata.type".as_ptr();

    #[link(name = "SDL3")]
    unsafe extern "C" {
        #[must_use]
        pub fn SDL_InitSubSystem(flags: c_uint) -> bool;

        pub fn SDL_Quit();

        #[must_use]
        pub fn SDL_SetAppMetadataProperty(
            name: *const c_char,
            value: *const c_char,
        ) -> bool;

        #[must_use]
        pub fn SDL_GetAppMetadataProperty(name: *const c_char)
        -> *const c_char;
    }
}

pub mod sdl_messagebox {
    use core::ffi::{c_char, c_uint};

    use super::sdl_video::SdlWindow;

    pub const SDL_MESSAGEBOX_ERROR: c_uint = 0x0000_0010;

    #[link(name = "SDL3")]
    unsafe extern "C" {
        #[must_use]
        pub fn SDL_ShowSimpleMessageBox(
            flags: c_uint,
            title: *const c_char,
            message: *const c_char,
            window: *mut SdlWindow,
        ) -> bool;
    }
}

pub mod sdl_video {
    #[link(name = "SDL3")]
    unsafe extern "C" {
        pub type SdlWindow;
    }
}
