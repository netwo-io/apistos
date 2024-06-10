use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) struct Schemas {
  pub(crate) deprecated: bool,
}

impl ToTokens for Schemas {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let deprecated = if self.deprecated {
      quote!(
        obj.insert("deprecated".to_owned(), true.into());
      )
    } else {
      quote!()
    };

    tokens.extend(quote! {
      fn child_schemas(oas_version: apistos::OpenApiVersion) -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::ApistosSchema>)> {
        let settings = match oas_version {
          apistos::OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3(),
          apistos::OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12(),
        };
        let mut gen = settings.into_generator();
        let mut schema: apistos::Schema = gen.into_root_schema_for::<Self>();

        let mut schemas: Vec<(String, apistos::reference_or::ReferenceOr<apistos::ApistosSchema>)> = vec![];
        let obj = schema.ensure_object();
        for (def_name, mut def) in obj
          .get("components")
          .and_then(|v| v.as_object())
          .and_then(|v| v.get("schemas"))
          .and_then(|v| v.as_object())
          .cloned()
          .unwrap_or_default()
        {
          let schema = apistos::Schema::try_from(def)
                    .map_err(|err| {
                      apistos::log::warn!("Error generating json schema: {err:?}");
                      err
                    })
                    .map(|sch| apistos::ApistosSchema::new(sch, oas_version))
                    .unwrap_or_default();
          schemas.push((def_name, apistos::reference_or::ReferenceOr::Object(schema)));
        }
        schemas
      }

      fn schema(oas_version: apistos::OpenApiVersion) -> Option<(String, apistos::reference_or::ReferenceOr<apistos::ApistosSchema>)> {
        let (name, schema) = {
          let schema_name = <Self as schemars::JsonSchema>::schema_name();
          let settings = match oas_version {
            apistos::OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3(),
            apistos::OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12(),
          };
          let mut gen = settings.into_generator();
          let mut schema: apistos::Schema = gen.into_root_schema_for::<Self>();

          let obj = schema.ensure_object();
          #deprecated
          let schema = apistos::ApistosSchema::new(apistos::Schema::from(obj.clone()), oas_version);
          (
            schema_name,
            apistos::reference_or::ReferenceOr::Object(schema)
          )
        };
        Some((name.to_string(), schema))
      }
    });
  }
}
