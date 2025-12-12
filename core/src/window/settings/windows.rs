//! Platform specific settings for Windows.

/// The platform specific window settings of an application.
#[derive(Debug, Clone)]
pub struct PlatformSpecific {
    /// Drag and drop support
    pub drag_and_drop: bool,

    /// Whether show or hide the window icon in the taskbar.
    pub skip_taskbar: bool,

    /// Shows or hides the background drop shadow for undecorated windows.
    ///
    /// The shadow is hidden by default.
    /// Enabling the shadow causes a thin 1px line to appear on the top of the window.
    pub undecorated_shadow: bool,

    /// Set the application's taskbar icon, also known as `ICON_BIG`. A reasonable ceiling
    /// is a size of 256x256px.
    pub taskbar_icon: Option<crate::window::Icon>,
}

impl Default for PlatformSpecific {
    fn default() -> Self {
        Self {
            drag_and_drop: true,
            skip_taskbar: false,
            undecorated_shadow: false,
            taskbar_icon: None,
        }
    }
}
