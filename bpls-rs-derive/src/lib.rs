use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, GenericParam, Generics, Ident, Lifetime, LifetimeDef, Visibility};

#[proc_macro_derive(Lexer, attributes(lexer, pattern))]
pub fn derive_lexer(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse
    let input = syn::parse_macro_input!(item as syn::ItemEnum);

    // Generate or get the lexer name
    let lexer_ident = create_lexer_name(&input.attrs, &input.ident);

    // Add the lifetime for the lexer
    let mut lexer_generics = input.generics.clone();
    lexer_generics
        .params
        .push(GenericParam::Lifetime(LifetimeDef::new(Lifetime::new(
            "\'__top",
            proc_macro2::Span::call_site(),
        ))));

    // Get the name of the end-of-input token
    let token_eoi = get_eoi();

    // Implement token type for the token
    let token_type_impl = add_token_type_impl(&token_eoi, &input.generics, &input.ident);

    // Add the struct for the lexer
    let lexer_struct = add_lexer_struct(&lexer_generics, &input.vis, &lexer_ident);

    // Add tokens enum implementation
    let tokens_impl = add_tokens_impl(&input.generics, &input.vis, &input.ident, &lexer_ident);

    // Add the lexer implementation
    let lexer_impl = add_lexer_impl(&lexer_generics, &input.vis, &lexer_ident);

    // Add the iterator implementation for the lexer
    let lexer_iter_impl = add_lexer_iter_impl(&lexer_generics, &input.ident, &lexer_ident);

    // Add the lexer trait implementation for the lexer
    let lexer_lexer_impl = add_lexer_lexer_impl(&lexer_generics, &input.ident, &lexer_ident);

    // Combine outputs
    (quote! {
        #token_type_impl
        #lexer_struct
        #tokens_impl
        #lexer_impl
        #lexer_iter_impl
        #lexer_lexer_impl
    })
    .into()
}

fn create_lexer_name(attrs: &Vec<Attribute>, identifier: &Ident) -> Ident {
    for attr in attrs {
        if let Some(p) = attr.path.segments.last() {
            if p.ident.to_string() == "lexer".to_owned() {
                return attr.parse_args().expect("invalid lexer name");
            }
        }
    }

    format_ident!("Lexer{}", identifier)
}

// TODO
fn get_eoi() -> Ident {}

fn add_token_type_impl(token_eoi: &Ident, generics: &Generics, ident: &Ident) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics TokenType<char> for #ident #ty_generics #where_clause {
            fn get_eoi() -> Self {
                ExampleToken::#token_eoi
            }
        }
    }
}

fn add_lexer_struct(
    lexer_generics: &Generics,
    vis: &Visibility,
    lexer_ident: &Ident,
) -> TokenStream {
    let a = quote! {
        // Struct for lexer
        #vis struct #lexer_ident #lexer_generics {
            input: ::std::str::CharIndices<'__top>,
            at: usize,
            c1: ::std::option::Option<(usize, char)>,
            c2: ::std::option::Option<(usize, char)>,
            done: bool,
        }
    };

    println!("a {:?}", a.to_string());

    a
}

fn add_tokens_impl(
    generics: &Generics,
    vis: &Visibility,
    ident: &Ident,
    lexer_ident: &Ident,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        // Implement for tokens enum
        impl #impl_generics #ident #ty_generics #where_clause {
            // TokensEnum::new_lexer_from_str() -> Lexer
            #vis fn new_lexer_from_str(input: &str) -> #lexer_ident {
                <#lexer_ident>::new_from_str(input)
            }
        }
    }
}

fn add_lexer_impl(lexer_generics: &Generics, vis: &Visibility, lexer_ident: &Ident) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = lexer_generics.split_for_impl();

    quote! {
        // Implement tokens for
        impl #impl_generics #lexer_ident #ty_generics #where_clause {
            #vis fn new_from_ci(input: ::std::str::CharIndices<'__top>) -> Self {
                // Initialize a new lexer
                let mut new = Self {
                    input,
                    at: 0,
                    c1: None,
                    c2: None,
                    done: false,
                };

                // Fill `current1` and `current2`
                new.next_char();
                new.next_char();

                // Return
                new
            }

            #vis fn new_from_str(input: &'__top str) -> Self {
                Self::new_from_ci(input.char_indices())
            }

            /// Increment this lexer by one character
            fn next_char(&mut self) {
                // Move the first to the second
                self.c2 = self.c1;

                // Update lexer position
                if let Some((at, _)) = self.c2 {
                    self.at = at;
                }

                // Get the next character
                self.c1 = self.input.next();
            }
        }
    }
}

fn add_lexer_iter_impl(
    lexer_generics: &Generics,
    ident: &Ident,
    lexer_ident: &Ident,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = lexer_generics.split_for_impl();

    quote! {
        impl #impl_generics ::std::iter::Iterator for #lexer_ident #ty_generics #where_clause {
            type Item = ::std::result::Result<Token<#ident>, LexingError<char>>;

            fn next(&mut self) -> ::std::option::Option<Self::Item> {
                if let ::std::option::Option::Some(c2) = self.c2 {
                    if let t_exp @ ::std::option::Option::Some(_) = match c2.1 {
                        // TODO:
                        _ => ::std::option::Option::Some(
                            ::std::result::Result::Err(
                                LexingError::UnexpectedInput {
                                    input: c2.1,
                                    index: c2.0,
                                }
                            )
                        ),
                    } {
                        self.next_char();
                        return t_exp;
                    }
                }

                if !self.done {
                    // The lexer is now finished but it hasn't output an end-of-input token yet
                    self.done = true;

                    // Increment to the nonexistent character input
                    self.at += 1;

                    // Output the end-of-input token
                    return ::std::option::Option::Some(
                        ::std::result::Result::Ok(
                            Token::new(
                                (self.at, self.at),
                                ExampleToken::get_eoi(),
                            )
                        )
                    );
                }
                ::std::option::Option::None
            }
        }
    }
}

fn add_lexer_lexer_impl(
    lexer_generics: &Generics,
    ident: &Ident,
    lexer_ident: &Ident,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = lexer_generics.split_for_impl();

    quote! {
        impl #impl_generics Lexer<char, #ident> for #lexer_ident #ty_generics #where_clause {
            fn get_pos(&self) -> usize {
                self.at
            }
        }
    }
}
