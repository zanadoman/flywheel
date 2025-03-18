use core::{
    ffi::CStr,
    ptr::null_mut,
    sync::atomic::{AtomicBool, Ordering},
};
use std::{ffi::CString, panic};

use super::ffi::sdl3::{sdl_error, sdl_init, sdl_messagebox};

static IS_CONTEXT_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Application data.
///
/// # Examples
///
/// ```
/// use flywheel::ContextData;
///
/// let context_data = ContextData {
///     name: "Game",
///     version: "0.1.0",
///     identifier: "com.example.game",
///     creator: "Example Studios",
///     copyright: "Copyright (C) 2025 Example Studios",
///     url: "game.example.com",
///     r#type: "game",
/// };
/// ```
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContextData<'a> {
    /// Name of the application.
    pub name: &'a str,
    /// Version of the application.
    pub version: &'a str,
    /// Identifier of the application.
    pub identifier: &'a str,
    /// Creator of the application.
    pub creator: &'a str,
    /// Copyright of the application.
    pub copyright: &'a str,
    /// URL of the application.
    pub url: &'a str,
    /// Type of the application.
    pub r#type: &'a str,
}

/// Application `Context`.
///
/// The `Context` initializes the underlying subsystems upon creation and
/// deinitializes them when dropped. Therefore, only one instance can be created
/// at a time.
///
/// # Examples
///
/// ```
/// use flywheel::{Context, ContextData};
///
/// let context = Context::new(&ContextData {
///     name: "Game",
///     version: "0.1.0",
///     identifier: "com.example.game",
///     creator: "Example Studios",
///     copyright: "Copyright (C) 2025 Example Studios",
///     url: "game.example.com",
///     r#type: "game",
/// }).unwrap();
/// ```
pub struct Context;

impl Context {
    /// Constructs a new application `Context`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the initialization fails. The
    /// `String` returned will contain details about the failure.
    pub fn new(context_data: &ContextData) -> Result<Self, String> {
        Self::set_panic_hook(context_data.name.to_owned());
        if IS_CONTEXT_INITIALIZED.swap(true, Ordering::SeqCst) {
            return Err("Cannot initialize the Context twice.".to_owned());
        }
        let name = CString::new(context_data.name).map_err(|err| {
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            err.to_string()
        })?;
        let version = CString::new(context_data.version).map_err(|err| {
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            err.to_string()
        })?;
        let identifier =
            CString::new(context_data.identifier).map_err(|err| {
                IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
                err.to_string()
            })?;
        let creator = CString::new(context_data.creator).map_err(|err| {
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            err.to_string()
        })?;
        let copyright =
            CString::new(context_data.copyright).map_err(|err| {
                IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
                err.to_string()
            })?;
        let url = CString::new(context_data.url).map_err(|err| {
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            err.to_string()
        })?;
        let r#type = CString::new(context_data.r#type).map_err(|err| {
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            err.to_string()
        })?;
        if !unsafe {
            sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_NAME_STRING,
                name.as_ptr(),
            ) && sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_VERSION_STRING,
                version.as_ptr(),
            ) && sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_IDENTIFIER_STRING,
                identifier.as_ptr(),
            ) && sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_CREATOR_STRING,
                creator.as_ptr(),
            ) && sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_COPYRIGHT_STRING,
                copyright.as_ptr(),
            ) && sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_URL_STRING,
                url.as_ptr(),
            ) && sdl_init::SDL_SetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_TYPE_STRING,
                r#type.as_ptr(),
            )
        } {
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            return Err(unsafe { CStr::from_ptr(sdl_error::SDL_GetError()) }
                .to_string_lossy()
                .to_string());
        }
        if !unsafe {
            sdl_init::SDL_InitSubSystem(
                sdl_init::SDL_INIT_AUDIO
                    | sdl_init::SDL_INIT_VIDEO
                    | sdl_init::SDL_INIT_EVENTS,
            )
        } {
            unsafe {
                sdl_init::SDL_Quit();
            }
            IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
            return Err(unsafe { CStr::from_ptr(sdl_error::SDL_GetError()) }
                .to_string_lossy()
                .to_string());
        }
        Ok(Self)
    }

    /// Returns the name of the application.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_NAME_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    /// Returns the version of the application.
    #[must_use]
    pub fn version(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_VERSION_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    /// Returns the identifier of the application.
    #[must_use]
    pub fn identifier(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_IDENTIFIER_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    /// Returns the creator of the application.
    #[must_use]
    pub fn creator(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_CREATOR_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    /// Returns the copyright of the application.
    #[must_use]
    pub fn copyright(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_COPYRIGHT_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    /// Returns the URL of the application.
    #[must_use]
    pub fn url(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_URL_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    /// Returns the type of the application.
    #[must_use]
    pub fn r#type(&self) -> String {
        unsafe {
            CStr::from_ptr(sdl_init::SDL_GetAppMetadataProperty(
                sdl_init::SDL_PROP_APP_METADATA_TYPE_STRING,
            ))
        }
        .to_string_lossy()
        .to_string()
    }

    fn set_panic_hook(title: String) {
        panic::set_hook(Box::new(move |panic_info| {
            let title = CString::new(title.clone())
                .unwrap_or_else(|_| c"Flywheel Engine".into());
            let message = CString::new(panic_info.to_string())
                .unwrap_or_else(|_| c"panic occurred".into());
            if !cfg!(test)
                && !unsafe {
                    sdl_messagebox::SDL_ShowSimpleMessageBox(
                        sdl_messagebox::SDL_MESSAGEBOX_ERROR,
                        title.as_ptr(),
                        message.as_ptr(),
                        null_mut(),
                    )
                }
            {
                eprintln!(
                    "{}",
                    unsafe { CStr::from_ptr(sdl_error::SDL_GetError()) }
                        .to_string_lossy()
                );
            }
            eprintln!("{panic_info}");
        }));
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sdl_init::SDL_Quit();
        }
        IS_CONTEXT_INITIALIZED.store(false, Ordering::SeqCst);
    }
}

impl !Send for Context {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        const CONTEXT_DATA: ContextData = ContextData {
            name: "Game",
            version: "0.1.0",
            identifier: "com.example.game",
            creator: "Example Studios",
            copyright: "Copyright (C) 2025 Example Studios",
            url: "game.example.com",
            r#type: "game",
        };
        let context = Context::new(&CONTEXT_DATA).unwrap();
        assert_eq!(context.name(), CONTEXT_DATA.name);
        assert_eq!(context.version(), CONTEXT_DATA.version);
        assert_eq!(context.identifier(), CONTEXT_DATA.identifier);
        assert_eq!(context.creator(), CONTEXT_DATA.creator);
        assert_eq!(context.copyright(), CONTEXT_DATA.copyright);
        assert_eq!(context.url(), CONTEXT_DATA.url);
        assert_eq!(context.r#type(), CONTEXT_DATA.r#type);
    }
}
