
use objc2::{
    declare_class,
    // msg_send,
    mutability,
    // sel,
    ClassType,
    DeclaredClass,
};
// use objc2::runtime::{
//     // Class,
//     // NSObject,
//     // Ivar,
//     Sel,
// };

use icrate::AuthenticationServices::ASCredentialProviderViewController;


// Declare your subclass


declare_class! {
    pub struct MyCredentialProviderViewController;

    unsafe impl ClassType for MyCredentialProviderViewController {
        type Super = ASCredentialProviderViewController;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyCredentialProviderViewController";
    }

    impl DeclaredClass for MyCredentialProviderViewController {}

    // unsafe impl ASCredentialProviderViewController for MyCredentialProviderViewController {
    //     #[method(prepareCredentialList:::)]
    //     fn prepare_credential_list(
    //         this: &Object,
    //         _cmd: Sel,
    //         service_identifiers: Id<NSArray, Strong>,
    //     ) {
    //         // Implement your logic for preparing the credential list
    //     }

    //     #[method(provideCredentialWithoutUserInteraction:::)]
    //     fn provide_credential_without_user_interaction(
    //         this: &Object,
    //         _cmd: Sel,
    //         credential_identity: Id<Object, Strong>,
    //     ) {
    //         // Implement your logic to provide a credential without user interaction
    //     }

    //     #[method(prepareInterfaceToProvideCredential:::)]
    //     fn prepare_interface_to_provide_credential(
    //         this: &Object,
    //         _cmd: Sel,
    //         credential_identity: Id<Object, Strong>,
    //     ) {
    //         // Implement your logic to prepare the interface for providing a specific credential
    //     }

    //     #[method(prepareInterfaceForExtensionConfiguration:::)]
    //     fn prepare_interface_for_extension_configuration(
    //         this: &Object,
    //         _cmd: Sel,
    //     ) {
    //         // Implement your logic for preparing the interface for extension configuration
    //     }
    // }
}
