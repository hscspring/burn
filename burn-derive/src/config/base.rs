use super::ConfigAnalyzerFactory;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn config_attr_impl(item: &syn::DeriveInput) -> TokenStream {
    let factory = ConfigAnalyzerFactory::new();
    let analyzer = factory.create_analyzer(item);

    let constructor = analyzer.gen_new_fn();
    let builders = analyzer.gen_builder_fns();
    let serde = analyzer.gen_serde_impl();
    let clone = analyzer.gen_clone_impl();
    let display = analyzer.gen_display_impl();
    let config_impl = analyzer.gen_config_impl();

    quote! {
        #config_impl
        #constructor
        #builders
        #serde
        #clone
        #display
    }
}
