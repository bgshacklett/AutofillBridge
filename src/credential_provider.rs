
// use objc2::declare::{
//     ClassBuilder,
// };
// use objc2::rc::{
//     Id,
// };
// use objc2::runtime::{
//     // Class,
//     // NSObject,
//     // Ivar,
//     Sel,
// };
use objc2::{
    // class,
    declare_class,
    // msg_send,
    mutability,
    // sel,
    ClassType,
    DeclaredClass,
};

use icrate::AuthenticationServices::ASCredentialProviderViewController;


// Declare your subclass
declare_class! {
    pub struct MyCredentialProviderViewController;

    unsafe impl ClassType for MyCredentialProviderViewController {
        type Super = ASCredentialProviderViewController;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyCredentialProviderViewController";
    }

    impl DeclaredClass for MyCredentialProviderViewController {
        // Your custom methods here
    }
}

// Register the subclass
// pub fn register_my_credential_provider_view_controller() {
//     unsafe {
//         let superclass = class!(ASCredentialProviderViewController);
//         let mut decl = ClassBuilder::new("MyCredentialProviderViewController", superclass).unwrap();

//         // Add methods
//         // decl.add_method(sel!(myCustomMethod:), my_custom_method as extern "C" fn(&MyCredentialProviderViewController, Sel));

//         // Register class
//         decl.register();
//     }
// }
