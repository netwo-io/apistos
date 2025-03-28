use std::collections::BTreeMap;

use apistos_models::paths::{MediaType, Response, Responses};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, OpenApiVersion, VersionSpecificSchema};

pub fn response_from_schema(
  oas_version: OpenApiVersion,
  status: &str,
  schema: Option<(String, ReferenceOr<ApistosSchema>)>,
) -> Option<Responses> {
  schema.map(|(_name, schema)| match schema {
    ReferenceOr::Reference { _ref } => Responses {
      responses: BTreeMap::from_iter(vec![(status.to_string(), ReferenceOr::Reference { _ref })]),
      ..Default::default()
    },
    ReferenceOr::Object(sch) => {
      let schema = match oas_version {
        OpenApiVersion::OAS3_0 => VersionSpecificSchema::OAS3_0(ReferenceOr::Object(sch)),
        OpenApiVersion::OAS3_1 => VersionSpecificSchema::OAS3_1(sch),
      };
      let response = Response {
        content: BTreeMap::from_iter(vec![(
          "application/json".to_string(),
          MediaType {
            schema: Some(schema),
            ..Default::default()
          },
        )]),
        ..Default::default()
      };
      Responses {
        responses: BTreeMap::from_iter(vec![(status.to_string(), ReferenceOr::Object(response))]),
        ..Default::default()
      }
    }
  })
}

pub fn response_from_raw_schema(
  oas_version: OpenApiVersion,
  status: &str,
  raw_schema: Option<ReferenceOr<ApistosSchema>>,
) -> Option<Responses> {
  raw_schema.map(|schema| match schema {
    ReferenceOr::Reference { _ref } => Responses {
      responses: BTreeMap::from_iter(vec![(status.to_string(), ReferenceOr::Reference { _ref })]),
      ..Default::default()
    },
    ReferenceOr::Object(sch) => {
      let schema = match oas_version {
        OpenApiVersion::OAS3_0 => VersionSpecificSchema::OAS3_0(sch.into()),
        OpenApiVersion::OAS3_1 => VersionSpecificSchema::OAS3_1(sch),
      };
      let response = Response {
        content: BTreeMap::from_iter(vec![(
          "application/json".to_string(),
          MediaType {
            schema: Some(schema),
            ..Default::default()
          },
        )]),
        ..Default::default()
      };
      Responses {
        responses: BTreeMap::from_iter(vec![(status.to_string(), ReferenceOr::Object(response))]),
        ..Default::default()
      }
    }
  })
}

pub fn response_for_status(status: &str) -> Responses {
  Responses {
    responses: BTreeMap::from_iter(vec![(status.to_string(), ReferenceOr::Object(Default::default()))]),
    ..Default::default()
  }
}
