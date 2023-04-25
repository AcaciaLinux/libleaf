use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Package)]
pub fn package_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_package_macro(&ast)
}

fn impl_package_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Package for #name {
            fn get_name(&self) -> String {
                self.name.to_owned()
            }
            fn set_name(&mut self, name: &str) {
                self.name = name.to_owned()
            }

            fn get_version(&self) -> String {
                self.version.to_owned()
            }
            fn set_version(&mut self, version: &str) {
                self.version = version.to_owned()
            }

            fn get_real_version(&self) -> u64 {
                self.real_version
            }
            fn set_real_version(&mut self, real_version: u64) {
                self.real_version = real_version
            }

            fn get_description(&self) -> &str {
                self.description.as_str()
            }
            fn set_description(&mut self, description: &str) {
                self.description = description.to_owned()
            }

            fn get_dependencies(&self) -> &Dependencies {
                &self.dependencies
            }
            fn set_dependencies(&mut self, dependencies: Dependencies) {
                self.dependencies = dependencies
            }

            fn get_hash(&self) -> String {
                self.hash.to_owned()
            }

            fn set_hash(&mut self, hash: &str) {
                self.hash = hash.to_owned()
            }
        }
    };
    gen.into()
}
