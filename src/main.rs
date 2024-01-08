#![deny(unsafe_op_in_unsafe_fn)]
use icrate::AppKit::{
    NSApplication,
    NSApplicationMain,
};
use icrate::Foundation::MainThreadMarker;

use objc2::runtime::ProtocolObject;
use objc2::rc::autoreleasepool;

use std::env;
use std::ffi::{CString, NulError};
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;
use std::process;

mod app_delegate;
mod credential_provider;


// Main entry point
fn main() -> Result<(), NulError> {
    embed_plist::embed_info_plist!("Info.plist");

    let args: Vec<String> = env::args().collect();

    let c_args = translate_args(args);

    let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);

    // configure the application delegate
    let delegate = app_delegate::AppDelegate::new(mtm.clone());
    let object = ProtocolObject::from_ref(delegate.as_ref());
    app.setDelegate(Some(object));


    let result = autoreleasepool(|_| {
        unsafe { NSApplicationMain(
            c_args.as_ref().unwrap().0,
            c_args.as_ref().unwrap().1,
        ) }
    });

    // You can handle the result if needed, or directly exit
    process::exit(result);
}


fn translate_args(args: Vec<String>) -> Result<(c_int, NonNull<NonNull<c_char>>), Box<dyn std::error::Error>> {
    let c_args: Vec<CString> = args.into_iter()
        .map(CString::new)
        .collect::<Result<Vec<_>, _>>()?;

    let argv: Vec<*const c_char> = c_args.iter()
        .map(|arg| arg.as_ptr())
        .collect();

    // Determine argc from the length of the argv vector
    let argc = argv.len() as i32;

    // Ensure all pointers are non-null and convert to NonNull<c_char>
    let non_null_argv: Vec<NonNull<c_char>> = argv.iter()
        .map(|&arg| NonNull::new(arg as *mut c_char).expect("Null pointer in argv"))
        .collect();

    // Convert the vector of NonNull<c_char> to NonNull<NonNull<c_char>>
    let argv = NonNull::new(non_null_argv.as_ptr() as *mut NonNull<c_char>)
        .expect("Null pointer in argv array");

    Ok((argc, argv))
}
