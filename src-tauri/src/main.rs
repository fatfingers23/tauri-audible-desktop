#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate objc;


#[cfg(target_os = "macos")]
use cocoa::appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable};
use cocoa::{
  base::{id, nil, NO, YES},
  foundation::{NSDictionary, NSFastEnumeration, NSInteger},
};

use std::{
  ffi::{c_void, CStr},
  os::raw::c_char,
  ptr::{null, null_mut},
  rc::Rc,
  slice, str,
  sync::{Arc, Mutex},
  fs
};


use tauri::{generate_context, Manager};

fn main() {
  tauri::Builder::default()
      .setup(|app| {
        let main_window = app.get_window("main").unwrap();
        main_window.eval("window.location.replace('https://www.audible.com/library')");

          app.listen_global("test-click", |event| {
              println!("got event-name with payload {:?}", event.payload());
          });

          app.listen_global("load", |event| {
              println!("Page loaded {:?}", event.payload());
          });
        main_window.with_webview(|webview| {
            //TODO sleepy. will load properly later
            //But this is hard coded each time you run npm run build need to update the file name. Super lazy i know but POC
          let main_script_content = fs::read_to_string("../dist/assets/index-8a110e02.js").expect("Unable to read main.js");

          #[cfg(target_os = "macos")]
          unsafe {
            let userscript: id = msg_send![class!(WKUserScript), alloc];
            let script:id = msg_send![userscript, initWithSource:NSString::new(main_script_content.as_str()) injectionTime:0 forMainFrameOnly:0];
            let _: () = msg_send![webview.controller(), addUserScript: script];
          }
        }).unwrap();
        Ok(())
      })
      .on_window_event(|event| {
        println!("event: {:?}", event)
      })
    .run(generate_context!())
    .expect("error while running tauri application");
}

const UTF8_ENCODING: usize = 4;

struct NSString(id);

impl NSString {
    fn new(s: &str) -> Self {
        // Safety: objc runtime calls are unsafe
        NSString(unsafe {
            let ns_string: id = msg_send![class!(NSString), alloc];
            let ns_string: id = msg_send![ns_string,
                            initWithBytes:s.as_ptr()
                            length:s.len()
                            encoding:UTF8_ENCODING];

            // The thing is allocated in rust, the thing must be set to autorelease in rust to relinquish control
            // or it can not be released correctly in OC runtime
            let _: () = msg_send![ns_string, autorelease];

            ns_string
        })
    }

    fn to_str(&self) -> &str {
        unsafe {
            let bytes: *const c_char = msg_send![self.0, UTF8String];
            let len = msg_send![self.0, lengthOfBytesUsingEncoding: UTF8_ENCODING];
            let bytes = slice::from_raw_parts(bytes as *const u8, len);
            str::from_utf8_unchecked(bytes)
        }
    }

    fn as_ptr(&self) -> id {
        self.0
    }
}