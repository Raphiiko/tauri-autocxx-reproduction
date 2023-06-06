// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autocxx::prelude::*;

include_cpp! {
    #include "openvr.h"
    generate!("vr::VR_Shutdown")
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// This command or function is never called!
async fn unsafe_function() {
    unsafe {
        ffi::vr::VR_Shutdown(); // When you comment this out, `tauri dev` works fine
    }
}
