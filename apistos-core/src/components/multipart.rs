use crate::ApiComponent;
use actix_multipart::Multipart;
use actix_multipart::form::text::Text;
use actix_multipart::form::{MultipartCollect, MultipartForm};
use apistos_models::ApistosSchema;
use apistos_models::paths::{MediaType, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

impl<T> ApiComponent for MultipartForm<T>
where
  T: MultipartCollect + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

impl<T> ApiComponent for Text<T>
where
  T: DeserializeOwned + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

impl<T> ApiComponent for actix_multipart::form::json::Json<T>
where
  T: DeserializeOwned + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

impl ApiComponent for Multipart {
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(_: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }

  fn raw_schema(_: apistos_models::OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    None
  }

  fn schema(_: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }

  fn request_body(_: apistos_models::OpenApiVersion, description: Option<String>) -> Option<RequestBody> {
    Some(RequestBody {
      content: BTreeMap::from_iter(vec![(Self::content_type(), MediaType::default())]),
      required: Some(Self::required()),
      description,
      ..Default::default()
    })
  }
}

#[cfg(feature = "multipart")]
pub mod tempfile {
  use crate::ApiComponent;
  use actix_multipart::form::{FieldReader, Limits};
  use actix_multipart::{Field, MultipartError};
  use actix_web::HttpRequest;
  use apistos_models::reference_or::ReferenceOr;
  use apistos_models::{ApistosSchema, OpenApiVersion};
  use futures_core::future::LocalBoxFuture;
  use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
  use std::borrow::Cow;

  #[derive(Debug)]
  pub struct Tempfile(actix_multipart::form::tempfile::TempFile);

  impl From<Tempfile> for actix_multipart::form::tempfile::TempFile {
    fn from(value: Tempfile) -> Self {
      value.0
    }
  }

  impl From<actix_multipart::form::tempfile::TempFile> for Tempfile {
    fn from(value: actix_multipart::form::tempfile::TempFile) -> Self {
      Self(value)
    }
  }

  impl JsonSchema for Tempfile {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "Tempfile".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "type": "string",
        "format": "binary"
      })
    }
  }

  impl ApiComponent for Tempfile {
    fn content_type() -> String {
      "multipart/form-data".to_string()
    }

    fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
      vec![]
    }

    fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
      let (name, schema) = {
        let schema_name = Self::schema_name();
        let settings = oas_version.get_schema_settings();
        let generator = settings.into_generator();
        let schema = generator.into_root_schema_for::<Self>();

        let schema = ApistosSchema::new(schema, oas_version);
        (schema_name, ReferenceOr::Object(schema))
      };
      Some((name.to_string(), schema))
    }
  }

  impl<'t> FieldReader<'t> for Tempfile {
    type Future = LocalBoxFuture<'t, Result<Self, MultipartError>>;

    fn read_field(req: &'t HttpRequest, field: Field, limits: &'t mut Limits) -> Self::Future {
      Box::pin(async move {
        actix_multipart::form::tempfile::TempFile::read_field(req, field, limits)
          .await
          .map(Into::into)
      })
    }
  }

  #[cfg(test)]
  mod test {
    use crate::ApiComponent;
    use crate::multipart::tempfile::Tempfile;
    use apistos_models::reference_or::ReferenceOr;
    use apistos_models::{ApistosSchema, OpenApiVersion};
    use assert_json_diff::assert_json_eq;
    use schemars::{JsonSchema, SchemaGenerator};
    use serde_json::json;

    #[test]
    #[expect(dead_code)]
    fn multipart_tempfile_schema() {
      #[derive(JsonSchema)]
      struct Test {
        file: Tempfile,
        label: String,
      }

      impl ApiComponent for Test {
        fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
          vec![]
        }

        fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
          let schema_settings = oas_version.get_schema_settings();
          let generator = SchemaGenerator::new(schema_settings);
          let schema = generator.into_root_schema_for::<Test>();
          Some(("Test".to_string(), ApistosSchema::new(schema, oas_version).into()))
        }
      }

      let schema = <Test as ApiComponent>::schema(OpenApiVersion::OAS3_0);
      let (schema_name, schema) = schema.expect("schema should be defined");
      assert_eq!(schema_name, "Test");
      let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
      assert_json_eq!(
        json,
        json!({
          "title": "Test",
          "type": "object",
          "properties": {
            "file": {
              "type": "string",
              "format": "binary"
            },
            "label": {
              "type": "string"
            }
          },
          "required": [
            "file",
            "label"
          ]
        })
      );

      let schema = <Test as ApiComponent>::schema(OpenApiVersion::OAS3_1);
      let (schema_name, schema) = schema.expect("schema should be defined");
      assert_eq!(schema_name, "Test");
      let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
      assert_json_eq!(
        json,
        json!({
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "title": "Test",
          "type": "object",
          "properties": {
            "file": {
              "type": "string",
              "format": "binary"
            },
            "label": {
              "type": "string"
            }
          },
          "required": [
            "file",
            "label"
          ]
        })
      );
    }
  }
}
