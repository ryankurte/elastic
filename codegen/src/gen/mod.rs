pub mod url_params;
pub mod url_builder;
pub mod request_params;
pub mod request_ctors;
pub mod request_into_http;

pub mod types {
    /// Type and declarations for the `Body` type.
    ///
    /// This type is an alias for a borrowed slice of bytes.
    pub mod body {
        use syn;
        use quote;
        use ::gen::helpers;

        pub fn ty() -> syn::Ty {
            helpers::ty_a("Body")
        }

        pub fn tokens() -> quote::Tokens {
            let body = ty();

            let from_vec = quote!(
                impl <'a> From<Vec<u8>> for #body {
                    fn from(value: Vec<u8>) -> #body {
                        Body(value.into())
                    }
                }
            );

            let from_slice = quote!(
                impl <'a> From<&'a [u8]> for #body {
                    fn from(value: &'a [u8]) -> #body {
                        Body(value.into())
                    }
                }
            );

            let from_str = quote!(
                impl <'a> From<&'a str> for #body {
                    fn from(value: &'a str) -> #body {
                        Body(value.as_bytes().into())
                    }
                }
            );

            let from_string = quote!(
                impl <'a> From<String> for #body {
                    fn from(value: String) -> #body {
                        Body(Cow::Owned(value.into()))
                    }
                }
            );

            let deref = quote!(
                impl <'a> Deref for #body {
                    type Target = Cow<'a, [u8]>;

                    fn deref(&self) -> &Cow<'a, [u8]> {
                        &self.0
                    }
                }
            );

            quote!(
                pub struct #body(Cow<'a, [u8]>);

                #from_vec
                
                #from_slice
                
                #from_str
                
                #from_string

                #deref
            )
        }
    }

    /// Type and declarations for the `Request` type.
    ///
    /// This type is a simple, standard wrapper for a HTTP request.
    pub mod request {
        use super::body;
        use syn;
        use ::gen::helpers;

        pub fn req_ty() -> syn::Ty {
            helpers::ty("HttpMethod")
        }

        pub fn method_ty() -> syn::Ty {
            helpers::ty_a("HttpRequest")
        }

        pub fn req_item() -> syn::Item {
            let r_ty = req_ty();

            helpers::parse_item(quote!(
                pub enum #r_ty {
                    Head,
                    Get,
                    Post,
                    Put,
                    Delete
                }
            ))
        }

        pub fn method_item() -> syn::Item {
            let r_ty = req_ty();
            let m_ty = method_ty();
            let b_ty = body::ty();

            helpers::parse_item(quote!(
                pub struct #m_ty {
                    pub url: Cow<'a, str>,
                    pub method: #r_ty,
                    pub body: Option<&'a #b_ty>
                }
            ))
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use ::gen::helpers::*;

            #[test]
            fn get_request_item() {
                let result = req_item();

                let expected = quote!(
                    pub enum HttpMethod {
                        Head,
                        Get,
                        Post,
                        Put,
                        Delete,
                    }
                );

                ast_eq(expected, result);
            }

            #[test]
            fn get_method_item() {
                let result = method_item();

                let expected = quote!(
                    pub struct HttpRequest<'a> {
                        pub url: Cow<'a, str>,
                        pub method: HttpMethod,
                        pub body: Option< Body<'a> >
                    }
                );

                ast_eq(expected, result);
            }
        }
    }

    /// Macro for declaring a wrapped type declaration.
    pub mod wrapped_ty {
        use inflector::Inflector;
        use quote;
        use ::gen::helpers;

        pub fn item(ty: &str) -> quote::Tokens {
            let ty = ty.to_pascal_case();

            let ident = helpers::ty(&ty);
            let ty = helpers::ty_a(&ty);

            let decl = quote!(
                    pub struct #ty(pub Cow<'a, str>)
                );

            let from_str = quote!(
                    impl <'a> From<&'a str> for #ty {
                        fn from(value: &'a str) -> #ty {
                            #ident(Cow::Borrowed(value))
                        }
                    }
                );

            let from_string = quote!(
                    impl <'a> From<String> for #ty {
                        fn from(value: String) -> #ty {
                            #ident(Cow::Owned(value))
                        }
                    }
                );

            let deref = quote!(
                    impl <'a> ::std::ops::Deref for #ty {
                        type Target = str;
                        
                        fn deref(&self) -> &str {
                            &self.0
                        }
                    }
                );

            quote!(
                    #decl;
                    #from_str
                    #from_string
                    #deref
                )
        }

        #[cfg(test)]
        pub mod tests {
            use super::item;
            use ::gen::helpers::*;

            #[test]
            fn gen_item() {
                let result = item("index");

                let decl = quote!(
                    pub struct Index<'a>(pub Cow<'a, str>)
                );

                let from_str = quote!(
                    impl <'a> From<&'a str> for Index<'a> {
                        fn from(value: &'a str) -> Index<'a> {
                            Index(Cow::Borrowed(value))
                        }
                    }
                );

                let from_string = quote!(
                    impl <'a> From<String> for Index<'a> {
                        fn from(value: String) -> Index<'a> {
                            Index(Cow::Owned(value))
                        }
                    }
                );

                let deref = quote!(
                    impl <'a> ::std::ops::Deref for Index<'a> {
                        type Target = str;
                        
                        fn deref(&self) -> &str {
                            &self.0
                        }
                    }
                );

                let expected = quote!(
                    #decl;
                    #from_str
                    #from_string
                    #deref
                );

                ast_eq(expected, result);
            }
        }
    }
}

pub mod helpers {
    use syn;
    use quote;
    use inflector::Inflector;

    fn sanitise_ident(ident: &str) -> &str {
        match ident {
            "type" => "ty",
            i => i,
        }
    }

    /// Build a sanitised `Ident`.
    pub fn ident<I: AsRef<str>>(ident: I) -> syn::Ident {
        let ident = ident.as_ref();

        syn::Ident::new(sanitise_ident(ident))
    }

    /// A standard `'a` lifetime.
    pub fn lifetime() -> syn::Lifetime {
        syn::Lifetime { ident: syn::Ident::new("'a") }
    }

    /// Generics with a standard `'a` lifetime.
    pub fn generics() -> syn::Generics {
        syn::Generics {
            lifetimes: vec![
                syn::LifetimeDef {
                    attrs: vec![],
                    lifetime: lifetime(),
                    bounds: vec![]
                }
            ],
            ty_params: vec![],
            where_clause: syn::WhereClause::none(),
        }
    }

    /// AST for a path type with a `'a` lifetime.
    pub fn ty_a(ty: &str) -> syn::Ty {
        ty_path(ty, vec![lifetime()], vec![])
    }

    /// AST for a simple path type.
    pub fn ty(ty: &str) -> syn::Ty {
        ty_path(ty, vec![], vec![])
    }

    /// AST for a path type with lifetimes and type parameters.
    pub fn ty_path(ty: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Ty>) -> syn::Ty {
        syn::Ty::Path(None,
                      syn::Path {
                          global: false,
                          segments: vec![
                                syn::PathSegment {
                                    ident: syn::Ident::new(ty),
                                    parameters: syn::PathParameters::AngleBracketed(
                                        syn::AngleBracketedParameterData {
                                            lifetimes: lifetimes,
                                            types: types,
                                            bindings: vec![]
                                        }
                                    )
                                }
                            ],
                      })
    }

    /// AST for a simple path variable.
    pub fn path(path: &str) -> syn::Path {
        syn::Path {
            global: false,
            segments: vec![syn::PathSegment::from(sanitise_ident(path))],
        }
    }

    /// AST for a simple method call.
    pub fn method(method: &str, args: Vec<&str>) -> syn::Expr {
        syn::ExprKind::MethodCall(ident(method),
                                  vec![],
                                  args.iter().map(|a| path(a).into_expr()).collect())
            .into()
    }

    /// AST for a simple field access.
    pub fn field(obj: &str, field: &str) -> syn::Expr {
        syn::ExprKind::Field(Box::new(path(obj).into_expr()), ident(field)).into()
    }

    /// Parse quoted tokens to an item.
    pub fn parse_item(input: quote::Tokens) -> syn::Item {
        syn::parse_item(input.to_string().as_ref()).unwrap()
    }

    /// Parse quoted tokens to an expression.
    pub fn parse_expr(input: quote::Tokens) -> syn::Expr {
        syn::parse_expr(input.to_string().as_ref()).unwrap()
    }

    /// Parse a name to a Rust PascalCase type name.
    pub trait IntoRustTypeName {
        fn into_rust_type(self) -> String;
    }

    impl<'a> IntoRustTypeName for &'a str {
        fn into_rust_type(self) -> String {
            str::replace(self, ".", "_").to_pascal_case()
        }
    }

    impl<'a> IntoRustTypeName for &'a syn::Ident {
        fn into_rust_type(self) -> String {
            (&self.to_string()).into_rust_type()
        }
    }

    /// Parse a name to a Rust snake_case variable name.
    pub trait IntoRustVarName {
        fn into_rust_var(self) -> String;
    }

    impl<'a> IntoRustVarName for &'a str {
        fn into_rust_var(self) -> String {
            let ident = self.split(".")
                .last()
                .unwrap()
                .to_snake_case();

            sanitise_ident(&ident).to_string()
        }
    }

    impl<'a> IntoRustVarName for &'a syn::Ident {
        fn into_rust_var(self) -> String {
            (&self.to_string()).into_rust_var()
        }
    }

    pub trait GetPath {
        fn get_path(&self) -> &syn::Path;
    }

    impl GetPath for syn::Ty {
        fn get_path(&self) -> &syn::Path {
            match self {
                &syn::Ty::Path(_, ref p) => &p,
                _ => panic!("Only path types are supported."),
            }
        }
    }

    impl GetPath for syn::Path {
        fn get_path(&self) -> &syn::Path {
            &self
        }
    }

    pub trait GetIdent {
        fn get_ident(&self) -> &syn::Ident;
    }

    impl<T: GetPath> GetIdent for T {
        fn get_ident(&self) -> &syn::Ident {
            &self.get_path().segments[0].ident
        }
    }

    /// Helper for wrapping a value as a quotable statement.
    pub trait IntoTy {
        fn into_ty(self) -> syn::Ty;
    }

    impl<T: GetPath> IntoTy for T {
        fn into_ty(self) -> syn::Ty {
            syn::Ty::Path(None, self.get_path().to_owned())
        }
    }

    /// Helper for wrapping a value as a quotable statement.
    pub trait IntoStmt {
        fn into_stmt(self) -> syn::Stmt;
    }

    impl IntoStmt for syn::Item {
        fn into_stmt(self) -> syn::Stmt {
            syn::Stmt::Item(Box::new(self))
        }
    }

    impl IntoStmt for syn::Expr {
        fn into_stmt(self) -> syn::Stmt {
            syn::Stmt::Expr(Box::new(self))
        }
    }

    /// Helper for wrapping a value as a quotable expression.
    pub trait IntoExpr {
        fn into_expr(self) -> syn::Expr;
    }

    impl IntoExpr for syn::Path {
        fn into_expr(self) -> syn::Expr {
            syn::ExprKind::Path(None, self).into()
        }
    }

    impl IntoExpr for syn::Block {
        fn into_expr(self) -> syn::Expr {
            syn::ExprKind::Block(syn::BlockCheckMode::Default, self).into()
        }
    }

    #[cfg(test)]
    pub fn ast_eq<T: quote::ToTokens>(expected: quote::Tokens, actual: T) {
        assert_eq!(expected.to_string(), quote!(#actual).to_string());
    }
}
