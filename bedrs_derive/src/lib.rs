extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Type};

// Helper structure to hold field information
#[derive(Default)]
struct FieldsInfo<'a> {
    chr: Option<&'a Type>,
    start: Option<&'a Type>,
    end: Option<&'a Type>,
    strand: Option<&'a Type>,
}

// Main entry point for the macro
#[proc_macro_derive(Coordinates, attributes(strand))]
pub fn coordinates_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = input.generics.clone();
    let mut ref_generics = input.generics.clone();
    ref_generics.params.insert(0, parse_quote!('a));

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (ref_impl_generics, _, _) = ref_generics.split_for_impl();

    let fields = parse_fields(&input);

    generate_impl(
        name,
        &fields,
        impl_generics,
        ty_generics,
        where_clause,
        ref_impl_generics,
    )
}

// Parses the fields and identifies the special ones
fn parse_fields(input: &DeriveInput) -> FieldsInfo {
    let mut fields_info = FieldsInfo::default();
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in fields_named.named.iter() {
                match field.ident.as_ref().unwrap().to_string().as_str() {
                    "chr" => fields_info.chr = Some(&field.ty),
                    "start" => fields_info.start = Some(&field.ty),
                    "end" => fields_info.end = Some(&field.ty),
                    "strand" => fields_info.strand = Some(&field.ty),
                    _ => {}
                }
            }
        }
    }
    fields_info
}

// Generates the implementation block
fn generate_impl(
    name: &syn::Ident,
    fields: &FieldsInfo,
    impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    ref_impl_generics: syn::ImplGenerics,
) -> TokenStream {
    let FieldsInfo {
        chr,
        start,
        end,
        strand,
    } = fields;
    if let (Some(chr), Some(_start), Some(_end)) = (chr, start, end) {
        let strand_method = generate_strand_method(strand);
        let update_strand_method = generate_update_strand_method(strand);
        let owned_impl = generate_impl_owned_block(
            name,
            chr,
            &strand_method,
            &update_strand_method,
            impl_generics,
            ty_generics.clone(),
            where_clause,
        );
        let ref_impl = generate_impl_ref_block(
            name,
            chr,
            &strand_method,
            ref_impl_generics.clone(),
            ty_generics.clone(),
            where_clause,
        );
        let mut_ref_impl = generate_impl_mut_ref_block(
            name,
            chr,
            &strand_method,
            &update_strand_method,
            ref_impl_generics.clone(),
            ty_generics.clone(),
            where_clause,
        );
        let expanded = quote! {
            #owned_impl
            #ref_impl
            #mut_ref_impl
        };
        TokenStream::from(expanded)
    } else {
        TokenStream::from(quote! {
            compile_error!("The struct must have `chr`, `start`, and `end` fields.");
        })
    }
}

/// Creates the owned implementation of `Coordinates`
#[allow(clippy::too_many_arguments)]
fn generate_impl_owned_block(
    name: &Ident,
    chr: &Type,
    strand_method: &proc_macro2::TokenStream,
    update_strand_method: &proc_macro2::TokenStream,
    impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> proc_macro2::TokenStream {
    quote! {
        impl #impl_generics Coordinates<#chr> for #name #ty_generics #where_clause {
            fn empty() -> Self {
                Self::default()
            }

            fn start(&self) -> i32 {
                self.start
            }

            fn end(&self) -> i32 {
                self.end
            }

            fn chr(&self) -> &#chr {
                &self.chr
            }

            fn update_start(&mut self, val: &i32) {
                self.start = *val;
            }

            fn update_end(&mut self, val: &i32) {
                self.end = *val;
            }

            fn update_chr(&mut self, val: &#chr) {
                self.chr = val.clone();
            }

            fn from<Iv: Coordinates<#chr>>(other: &Iv) -> Self {
                let mut new = Self::empty();
                new.update_chr(other.chr());
                new.update_start(&other.start());
                new.update_end(&other.end());
                new.update_strand(other.strand());
                new
            }

            #strand_method

            #update_strand_method
        }
    }
}

/// Creates the immutable reference implementation of `Coordinates`
#[allow(clippy::too_many_arguments)]
fn generate_impl_ref_block(
    name: &Ident,
    chr: &Type,
    strand_method: &proc_macro2::TokenStream,
    ref_impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> proc_macro2::TokenStream {
    quote! {
        impl #ref_impl_generics Coordinates<#chr> for &'a #name #ty_generics #where_clause {
            fn empty() -> Self {
                unimplemented!("Cannot create an empty interval as an immutable reference")
            }

            fn start(&self) -> i32 {
                self.start
            }

            fn end(&self) -> i32 {
                self.end
            }

            fn chr(&self) -> &#chr {
                &self.chr
            }

            fn update_start(&mut self, val: &i32) {
                unimplemented!("Cannot update an immutable reference")
            }

            fn update_end(&mut self, val: &i32) {
                unimplemented!("Cannot update an immutable reference")
            }

            fn update_chr(&mut self, val: &#chr) {
                unimplemented!("Cannot update an immutable reference")
            }

            fn update_strand(&mut self, _val: Option<Strand>) {
                unimplemented!("Cannot update an immutable reference")
            }

            fn from<Iv: Coordinates<#chr>>(other: &Iv) -> Self {
                unimplemented!("Cannot create a new reference from a reference")
            }

            #strand_method
        }
    }
}

/// Creates the mutable reference implementation of `Coordinates`
#[allow(clippy::too_many_arguments)]
fn generate_impl_mut_ref_block(
    name: &Ident,
    chr: &Type,
    strand_method: &proc_macro2::TokenStream,
    update_strand_method: &proc_macro2::TokenStream,
    ref_impl_generics: syn::ImplGenerics,
    ty_generics: syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
) -> proc_macro2::TokenStream {
    quote! {
        impl #ref_impl_generics Coordinates<#chr> for &'a mut #name #ty_generics #where_clause {
            fn empty() -> Self {
                unimplemented!("Cannot create an empty interval as an immutable reference")
            }

            fn start(&self) -> i32 {
                self.start
            }

            fn end(&self) -> i32 {
                self.end
            }

            fn chr(&self) -> &#chr {
                &self.chr
            }

            fn update_start(&mut self, val: &i32) {
                self.start = *val;
            }

            fn update_end(&mut self, val: &i32) {
                self.end = *val;
            }

            fn update_chr(&mut self, val: &#chr) {
                self.chr = val.clone();
            }

            fn from<Iv: Coordinates<#chr>>(other: &Iv) -> Self {
                unimplemented!("Cannot create a new reference from a reference")
            }

            #strand_method

            #update_strand_method

        }
    }
}

// Generates the method for the optional `strand` field
fn generate_strand_method(strand: &Option<&Type>) -> proc_macro2::TokenStream {
    match strand {
        Some(_strand) => quote! {
            fn strand(&self) -> Option<Strand> {
                Some(self.strand)
            }

        },
        None => quote! {
            fn strand(&self) -> Option<Strand> {
                None
            }
        },
    }
}

// Generates the method for the optional `strand` field
fn generate_update_strand_method(strand: &Option<&Type>) -> proc_macro2::TokenStream {
    match strand {
        Some(_strand) => quote! {
            fn update_strand(&mut self, val: Option<Strand>) {
                if let Some(val) = val {
                    self.strand = val;
                } else {
                    self.strand = Strand::Unknown;
                }
            }

        },
        None => quote! {
            fn update_strand(&mut self, _val: Option<Strand>) {
                // Do nothing
            }
        },
    }
}
