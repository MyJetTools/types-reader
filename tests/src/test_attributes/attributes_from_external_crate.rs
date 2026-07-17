#[cfg(test)]
mod tests {
    use types_reader_core::Attributes;

    fn parse(src: &str) -> syn::DeriveInput {
        syn::parse_str(src).unwrap()
    }

    /// Attributes has to be nameable and constructible from the outside of the types-reader-core crate
    #[test]
    fn test_container_attributes_are_readable_from_external_crate() {
        let ast = parse(
            r#"
            #[serde(rename_all = "camelCase")]
            pub struct MyModel {
                pub id: u32,
            }
        "#,
        );

        let attrs: Attributes = Attributes::new(&ast, &ast.attrs).unwrap();

        assert_eq!(
            attrs
                .get_named_param("serde", "rename_all")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "camelCase"
        );
    }

    /// The list form of the parameter must not be lost silently
    #[test]
    fn test_serde_rename_all_in_list_form_is_readable_from_external_crate() {
        let ast = parse(
            r#"
            #[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
            pub struct MyModel {
                pub id: u32,
            }
        "#,
        );

        let attrs = Attributes::new(&ast, &ast.attrs).unwrap();

        let rename_all = attrs.get_named_param("serde", "rename_all").unwrap();

        assert_eq!(
            rename_all
                .get_named_param("serialize")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "camelCase"
        );

        assert_eq!(
            rename_all
                .get_named_param("deserialize")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "kebab-case"
        );
    }

    /// Broken attribute has to give a syn::Error - not to panic
    #[test]
    fn test_broken_attribute_returns_error_and_does_not_panic() {
        let ast = parse(
            r#"
            #[my_attr(id =)]
            pub struct MyModel {
                pub id: u32,
            }
        "#,
        );

        let result = Attributes::new(&ast, &ast.attrs);

        assert!(result.is_err());
    }
}
