Creating a custom derive macro in Rust involves using procedural macros. Here’s a step-by-step guide to writing a `#[derive(Validate)]` macro that validates struct fields based on custom attributes.

---

## **Step 1: Create a Procedural Macro Library**

You need a separate crate for procedural macros:

```sh
cargo new validate_macro --lib
cd validate_macro
cargo add syn quote proc-macro2
```

Then, enable the procedural macro feature by adding this to `Cargo.toml`:

```toml
[lib]
proc-macro = true
```

---

## **Step 2: Implement the `#[derive(Validate)]` Macro**

Inside `src/lib.rs`, write the macro:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Validate)]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> Result<(), String> {
                // Custom validation logic here
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
```

---

## **Step 3: Use the Macro in a Project**

Create a separate Rust project and add `validate_macro` as a dependency:

```sh
cargo new validate_test
cd validate_test
```

Edit `Cargo.toml` to include:

```toml
[dependencies]
validate_macro = { path = "../validate_macro" }
```

Now, use the macro in `src/main.rs`:

```rust
use validate_macro::Validate;

#[derive(Validate)]
struct User {
    name: String,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
    };

    if let Err(e) = user.validate() {
        println!("Validation failed: {}", e);
    } else {
        println!("Validation passed!");
    }
}
```

---

## **Step 4: Adding Custom Field Attributes**

To validate fields, modify the macro to accept custom attributes.

Update `lib.rs`:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Validate, attributes(min_length))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let mut field_checks = vec![];

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            for field in fields.named {
                let field_name = field.ident.unwrap();

                for attr in field.attrs {
                    if attr.path().is_ident("min_length") {
                        let meta = attr.parse_meta().unwrap();
                        if let syn::Meta::NameValue(nv) = meta {
                            if let syn::Expr::Lit(lit) = nv.value {
                                if let syn::Lit::Int(len) = lit.lit {
                                    let min_length: usize = len.base10_parse().unwrap();
                                    field_checks.push(quote! {
                                        if self.#field_name.len() < #min_length {
                                            return Err(format!("{} must be at least {} characters long", stringify!(#field_name), #min_length));
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> Result<(), String> {
                #(#field_checks)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
```

---

## **Step 5: Use the Custom Attribute in Your Struct**

Now, modify `validate_test/src/main.rs`:

```rust
use validate_macro::Validate;

#[derive(Validate)]
struct User {
    #[min_length = 3]
    name: String,
}

fn main() {
    let user = User {
        name: "Al".to_string(),
    };

    match user.validate() {
        Ok(_) => println!("Validation passed!"),
        Err(e) => println!("Validation failed: {}", e),
    }
}
```

---

### **Summary**

- Created a `#[derive(Validate)]` macro.
- Added support for `#[min_length = N]` field attributes.
- Used `syn` to parse struct fields and extract attribute values.
- Generated a `validate()` method for runtime validation.

Would you like further improvements like supporting more validations (`max_length`, `email`, etc.)? 🚀
