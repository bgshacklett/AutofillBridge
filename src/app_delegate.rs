#![deny(unsafe_op_in_unsafe_fn)]
use icrate::AppKit::{
    NSMenu,
    NSMenuItem,
    NSSquareStatusItemLength,
    NSStatusBar,
    NSApplicationDelegate, NSStatusItem,
};
use icrate::Foundation::{
    NSString,
    MainThreadMarker,
    NSNotification,
    NSObject,
    NSObjectProtocol,
};

use objc2::rc::Id;
use objc2::{
    sel,
    declare_class,
    msg_send_id,
    mutability,
    ClassType,
    DeclaredClass,
};

use crate::credential_provider;


#[derive(Debug)]
#[allow(unused)]
pub struct Ivars {
    mtm: MainThreadMarker,
    main_status_item: Id<NSStatusItem>,
}


declare_class!(
    pub struct AppDelegate;

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
        fn did_finish_launching(&self, _notification: &NSNotification) {
            let ivars = self.ivars();
            let mtm = &ivars.mtm;
            let main_status_item = &ivars.main_status_item;

            unsafe {
                main_status_item
                    .button(*mtm)
                    .expect("Could not retrieve button from NSStatusItem")
                    .setTitle(NSString::from_str("A").as_ref())
            };

            let menu = NSMenu::new(*mtm);
            unsafe { main_status_item.setMenu(Some(menu.as_ref())) };

            let title_str = NSString::from_str("Quit");

            let menu_item = NSMenuItem::new(*mtm);
            unsafe { menu_item.setTitle(title_str.as_ref()) };
            unsafe { menu_item.setAction(Some(sel!(terminate:))) };

            menu.addItem(menu_item.as_ref());
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

        let status_bar = unsafe { NSStatusBar::systemStatusBar() };
        let main_status_item = unsafe {
            status_bar.statusItemWithLength(NSSquareStatusItemLength)
        };

        let this = this.set_ivars(Ivars {
            mtm,
            main_status_item,
        });

        unsafe { msg_send_id![super(this), init] }
    }
}
