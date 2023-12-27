#![deny(unsafe_op_in_unsafe_fn)]
use icrate::AppKit::{
    NSMenu,
    NSImage,
    // NSVariableStatusItemLength,
    NSStatusBar,
    NSApplication,
    NSApplicationDelegate,
    NSMenuItem,
    NSSquareStatusItemLength,
};
use icrate::Foundation::{
    MainThreadMarker,
    NSNotification,
    NSObject,
    NSObjectProtocol,
    NSString,
};

use objc2::runtime::{ProtocolObject};
use objc2::rc::Id;
use objc2::{
    sel,
    msg_send,
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
            let ivars = self.ivars();
            let mtm = &ivars.mtm;

            let status_bar = unsafe { NSStatusBar::systemStatusBar() };
            let main_status_item = unsafe {
                status_bar.statusItemWithLength(NSSquareStatusItemLength)
            };

            unsafe {
                main_status_item.button(*mtm).expect("REASON").setImage(
                    NSImage::imageNamed(&*NSString::from_str("test")).as_deref()
                );
            }

            let menu = NSMenu::new(*mtm);

            let title_str = NSString::from_str("Do Something");
            let key_equiv_str = NSString::from_str("");

            let alloc_menu_item: *mut NSMenuItem = unsafe {
                msg_send![NSMenuItem::class(), alloc]
            };
            let menu_item: *mut NSMenuItem = unsafe {
                msg_send![alloc_menu_item, initWithTitle:title_str.as_ref(),
                                           action:sel!(doSomething:),
                                           keyEquivalent:key_equiv_str.as_ref()]
            };
            let menu_item: Id<NSMenuItem> = unsafe {
                Id::new(menu_item).unwrap()
            };

            unsafe {main_status_item.setMenu(Some(menu.as_ref()))};

            menu.addItem(menu_item.as_ref());

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
