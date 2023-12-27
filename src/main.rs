#![deny(unsafe_op_in_unsafe_fn)]
use icrate::AppKit::{
    NSApplication,
    NSApplicationDelegate,
};
use icrate::Foundation::{
    MainThreadMarker,
    NSNotification,
    NSObject,
    NSObjectProtocol,
};

use objc2::runtime::ProtocolObject;
use objc2::rc::Id;
use objc2::{
    declare_class,
    msg_send_id,
    mutability,
    ClassType,
    DeclaredClass,
};


#[derive(Debug)]
#[allow(unused)]
struct Ivars {
    mtm: MainThreadMarker,
}

declare_class!(
    struct AppDelegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application
    //   delegate.
    // - `AppDelegate` does not implement `Drop`.
    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyAppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, notification: &NSNotification) {
            // Do something with the notification
            dbg!(notification);
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
            println!("Will terminate!");
        }
    }
);

impl AppDelegate {
    pub fn new(mtm: MainThreadMarker) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {
            mtm,
        });
        unsafe { msg_send_id![super(this), init] }
    }
}


// Main entry point
fn main() {
    let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();

    let app = NSApplication::sharedApplication(mtm);






    // configure the application delegate
    let delegate = AppDelegate::new(mtm.clone());
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    unsafe { app.run() };
}
