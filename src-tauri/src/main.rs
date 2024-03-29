// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autocxx::prelude::*;

include_cpp! {
    #include "openvr.h"
    generate!("vr::VR_Shutdown")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![unsafe_function])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Note that this command or function is never called. It is only here to demonstrate the issue.
#[tauri::command]
async fn unsafe_function() {
    unsafe {
        ffi::vr::VR_Shutdown(); //L22: When you comment this out, `tauri dev` works fine
    }
}
