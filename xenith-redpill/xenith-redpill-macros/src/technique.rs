use convert_case::{Case, Casing};
use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Arguments for the `#[technique]` attribute macro
/// This struct is used to parse the arguments provided to the `#[technique]` attribute macro.
#[derive(Debug, FromMeta)]
struct TechniqueArgs {
    name: String,
    description: String,
    os: String, // todo: enum
}

pub fn uppercase_first_letter(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Implements the `#[technique]` attribute macro
/// This macro is used to define a new detection technique.
///
/// # Arguments
///
/// * `name` - The name of the technique
/// * `description` - A description of the technique
/// * `os` - The operating system(s) the technique is compatible with
///
/// # Returns
///
/// A new technique that can be used to detect the presence of the Xen hypervisor.
/// This is a TokenStream that contains the implementation of the technique. The
/// technique is automatically registered with the technique registry when the module
/// is loaded and also executed when the registry is queried for techniques.
pub fn technique_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let input = syn::parse_macro_input!(input as ItemFn);

    let args = match TechniqueArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let function_name = &input.sig.ident;
    let uppercase_function_name = syn::Ident::new(
        &function_name.to_string().to_uppercase(),
        function_name.span(),
    );

    let function_content = &input.block;
    let technique_name = &args.name;
    let technique_description = &args.description;
    let technique_os = &args.os;

    let function_name_as_camel_case = function_name.to_string().to_case(Case::Camel);
    let struct_name = syn::Ident::new(
        &format!(
            "{}Technique",
            uppercase_first_letter(function_name_as_camel_case)
        ),
        function_name.span(),
    );

    let os_cfg = match technique_os.as_str() {
        "linux" => quote! { #[cfg(target_os = "linux")] },
        "windows" => quote! { #[cfg(target_os = "windows")] },
        "all" => quote! {},
        _ => {
            return TokenStream::from(
                Error::custom("Invalid OS, choose from 'linux', 'windows' or 'all'").write_errors(),
            );
        }
    };

    let expanded = quote! {
        #[doc = concat!(
            "# ", #technique_name, "\n\n",
            "## Description\n\n",
            #technique_description, "\n\n",
            "## Operating system compatibility\n\n",
            "This technique is compatible with the following operating systems: ", #technique_os, "\n"
        )]
        #os_cfg
        pub fn #function_name() -> TechniqueResult {
            #function_content
        }

        #os_cfg
        struct #struct_name;

        #os_cfg
        impl Technique for #struct_name {
            fn name(&self) -> &'static str {
                #technique_name
            }
            fn description(&self) -> &'static str {
                #technique_description
            }
            fn execute(&self) -> TechniqueResult {
                #function_name()
            }
        }

        #os_cfg
        #[dynamic]
        static #uppercase_function_name: () = {
            let res = register_technique(#struct_name);
            if let Err(e) = res {
                error!("Failed to register technique: {}", e);
            }
        };
    };

    expanded.into()
}
