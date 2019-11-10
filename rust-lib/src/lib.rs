extern crate sdl2;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use sdl2::event::Event;
use sdl2::rect::Point;

#[macro_use]
mod macros;
mod tetris;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate android_log;
    extern crate jni;

    use jni::sys::jint;
    use jni::JavaVM;
    use std::os::raw::c_void;
    use std::sync::{Arc, Once};

    static mut JVM: Option<Arc<JavaVM>> = None;
    static INIT: Once = Once::new();

    pub fn jvm() -> &'static Arc<JavaVM> {
        unsafe { JVM.as_ref().unwrap() }
    }

    #[no_mangle]
    pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, _reserved: *mut c_void) -> jint {
        android_log::init("Rust SDL2").unwrap();
        INIT.call_once(|| unsafe {
            JVM = Some(Arc::new(vm));
        });
        jni::sys::JNI_VERSION_1_6
    }

    pub fn get_resolution() -> (u32, u32) {
        let env = jvm().attach_current_thread_as_daemon().unwrap();
        let resolution = env
            .call_static_method(
                "hoangpq/hello_sdl2/HelloSDL2Activity",
                "getResolution",
                "()Landroid/util/DisplayMetrics;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();

        let width_pixels = env.get_field(resolution, "widthPixels", "I").unwrap();
        let height_pixels = env.get_field(resolution, "heightPixels", "I").unwrap();

        (
            width_pixels.i().unwrap() as u32,
            height_pixels.i().unwrap() as u32,
        )
    }
}

#[cfg(target_os = "ios")]
#[allow(non_snake_case)]
pub mod ios {
    pub fn get_resolution() -> (u32, u32) {
        (1, 1)
    }
}

#[cfg(target_os = "android")]
pub fn get_resolution() -> (u32, u32) {
    android::get_resolution()
}

#[cfg(target_os = "ios")]
pub fn get_resolution() -> (u32, u32) {
    ios::get_resolution()
}

#[no_mangle]
pub extern "C" fn sdl2_main() {
    let (width, height) = get_resolution();
    sdl2_log!("({}, {})", width, height);
    #[cfg(target_os = "android")]
    tetris::main(width, height);
}
