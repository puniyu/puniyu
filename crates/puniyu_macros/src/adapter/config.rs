use syn::DeriveInput;

use crate::config::{ConfigContext, derive_config_impl};

pub fn derive_adapter_config(input: DeriveInput) -> proc_macro2::TokenStream {
	derive_config_impl(input, ConfigContext::Adapter)
}
