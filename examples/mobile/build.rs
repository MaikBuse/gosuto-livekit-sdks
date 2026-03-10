use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        gosuto_webrtc_sys_build::configure_jni_symbols().unwrap();
    }
}
