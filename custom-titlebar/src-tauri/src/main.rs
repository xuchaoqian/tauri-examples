#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{window::WindowBuilder, AppHandle, LogicalSize, Size, TitleBarStyle, WindowUrl};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use tauri::{Runtime, Window};

use cocoa::appkit::NSWindowTitleVisibility;

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;
            NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                title_transparent,
            );

            if remove_tool_bar {
                style_mask.remove(
                    NSWindowStyleMask::NSClosableWindowMask
                        | NSWindowStyleMask::NSMiniaturizableWindowMask
                        | NSWindowStyleMask::NSResizableWindowMask,
                );
            }

            id.setStyleMask_(style_mask);

            id.setTitleVisibility_(if title_transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });

            id.setTitlebarAppearsTransparent_(if title_transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            create_splash_window(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn create_splash_window(app_handle: AppHandle) {
    // tauri::async_runtime::spawn(async move {

    // });

    let window = WindowBuilder::new(&app_handle, "_splash", WindowUrl::App("splash.html".into()))
        .title("Custom Titlebar")
        .inner_size(320.0, 200.0)
        // .decorations(false)
        // .title_bar_style(TitleBarStyle::Overlay)
        // .hidden_title(true)
        // .transparent(false)
        .resizable(true)
        .visible(false)
        .build()
        .unwrap();
    let size = LogicalSize {
        width: 320.0,
        height: 200.0,
    };
    window
        .set_size(Size::Logical(size))
        .unwrap_or_else(|err| println!("Failed to set size, error: {:?}", err));

    window.set_transparent_titlebar(true, true);

    // #[cfg(target_os = "macos")]
    // apply_vibrancy(&window, NSVisualEffectMaterial::Tooltip, None, Some(128.0))
    //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    window
        .show()
        .unwrap_or_else(|err| println!("Failed to show, error: {:?}", err));
}
