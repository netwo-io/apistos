#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::api::test::todo::get_todo;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::{OpenApiVersion, RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig};
use std::error::Error;
use std::net::Ipv4Addr;
mod api {
    use actix_web::scope;
    pub mod test {
        use actix_web::scope;
        pub mod todo {
            use actix_web::web::{Json, Path};
            use actix_web::Error;
            use apistos::{get, post, ApiComponent};
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use uuid::Uuid;
            pub struct NewTodo {
                pub title: String,
                pub description: Option<String>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for NewTodo {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "title" => _serde::__private::Ok(__Field::__field0),
                                    "description" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"title" => _serde::__private::Ok(__Field::__field0),
                                    b"description" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<NewTodo>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = NewTodo;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct NewTodo",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct NewTodo with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct NewTodo with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(NewTodo {
                                    title: __field0,
                                    description: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<String>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "description",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("title")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("description")?
                                    }
                                };
                                _serde::__private::Ok(NewTodo {
                                    title: __field0,
                                    description: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "title",
                            "description",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "NewTodo",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<NewTodo>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for NewTodo {
                    fn schema_name() -> schemars::_private::alloc::borrow::Cow<
                        'static,
                        str,
                    > {
                        schemars::_private::alloc::borrow::Cow::Borrowed("NewTodo")
                    }
                    fn schema_id() -> schemars::_private::alloc::borrow::Cow<
                        'static,
                        str,
                    > {
                        schemars::_private::alloc::borrow::Cow::Borrowed(
                            "simple_todo_macros::api::test::todo::NewTodo",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::SchemaGenerator,
                    ) -> schemars::Schema {
                        {
                            let mut schema = ::schemars::Schema::try_from(
                                    ::serde_json::Value::Object({
                                        let mut object = ::serde_json::Map::new();
                                        let _ = object
                                            .insert(
                                                ("type").into(),
                                                ::serde_json::to_value(&"object").unwrap(),
                                            );
                                        object
                                    }),
                                )
                                .unwrap();
                            {
                                schemars::_private::insert_object_property(
                                    &mut schema,
                                    "title",
                                    if generator.contract().is_serialize() {
                                        false
                                    } else {
                                        false
                                            || (!false
                                                && <String as schemars::JsonSchema>::_schemars_private_is_option())
                                    },
                                    { generator.subschema_for::<String>() },
                                );
                            }
                            {
                                schemars::_private::insert_object_property(
                                    &mut schema,
                                    "description",
                                    if generator.contract().is_serialize() {
                                        false
                                    } else {
                                        false
                                            || (!false
                                                && <Option<
                                                    String,
                                                > as schemars::JsonSchema>::_schemars_private_is_option())
                                    },
                                    { generator.subschema_for::<Option<String>>() },
                                );
                            }
                            schema
                        }
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                impl apistos::ApiComponent for NewTodo {
                    fn child_schemas(
                        oas_version: apistos::OpenApiVersion,
                    ) -> Vec<
                        (
                            String,
                            apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                        ),
                    > {
                        let settings = oas_version.get_schema_settings();
                        let mut gen = settings.into_generator();
                        let mut schema: apistos::Schema = gen
                            .into_root_schema_for::<Self>();
                        let mut schemas: Vec<
                            (
                                String,
                                apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                            ),
                        > = ::alloc::vec::Vec::new();
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
                                    {
                                        let lvl = ::log::Level::Warn;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!("Error generating json schema: {0:?}", err),
                                                lvl,
                                                &(
                                                    "simple_todo_macros::api::test::todo",
                                                    "simple_todo_macros::api::test::todo",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                    err
                                })
                                .map(|sch| apistos::ApistosSchema::new(sch, oas_version))
                                .unwrap_or_default();
                            schemas
                                .push((
                                    def_name,
                                    apistos::reference_or::ReferenceOr::Object(schema),
                                ));
                        }
                        schemas
                    }
                    fn schema(
                        oas_version: apistos::OpenApiVersion,
                    ) -> Option<
                        (
                            String,
                            apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                        ),
                    > {
                        let (name, schema) = {
                            let schema_name = <Self as schemars::JsonSchema>::schema_name();
                            let settings = oas_version.get_schema_settings();
                            let mut gen = settings.into_generator();
                            let mut schema: apistos::Schema = gen
                                .into_root_schema_for::<Self>();
                            let obj = schema.ensure_object();
                            let schema = apistos::ApistosSchema::new(
                                apistos::Schema::from(obj.clone()),
                                oas_version,
                            );
                            (
                                schema_name,
                                apistos::reference_or::ReferenceOr::Object(schema),
                            )
                        };
                        Some((name.to_string(), schema))
                    }
                }
            };
            pub struct Todo {
                pub id: Uuid,
                pub title: String,
                pub description: Option<String>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Todo {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Todo",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "title",
                            &self.title,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "description",
                            &self.description,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for Todo {
                    fn schema_name() -> schemars::_private::alloc::borrow::Cow<
                        'static,
                        str,
                    > {
                        schemars::_private::alloc::borrow::Cow::Borrowed("Todo")
                    }
                    fn schema_id() -> schemars::_private::alloc::borrow::Cow<
                        'static,
                        str,
                    > {
                        schemars::_private::alloc::borrow::Cow::Borrowed(
                            "simple_todo_macros::api::test::todo::Todo",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::SchemaGenerator,
                    ) -> schemars::Schema {
                        {
                            let mut schema = ::schemars::Schema::try_from(
                                    ::serde_json::Value::Object({
                                        let mut object = ::serde_json::Map::new();
                                        let _ = object
                                            .insert(
                                                ("type").into(),
                                                ::serde_json::to_value(&"object").unwrap(),
                                            );
                                        object
                                    }),
                                )
                                .unwrap();
                            {
                                schemars::_private::insert_object_property(
                                    &mut schema,
                                    "id",
                                    if generator.contract().is_serialize() {
                                        false
                                    } else {
                                        false
                                            || (!false
                                                && <Uuid as schemars::JsonSchema>::_schemars_private_is_option())
                                    },
                                    { generator.subschema_for::<Uuid>() },
                                );
                            }
                            {
                                schemars::_private::insert_object_property(
                                    &mut schema,
                                    "title",
                                    if generator.contract().is_serialize() {
                                        false
                                    } else {
                                        false
                                            || (!false
                                                && <String as schemars::JsonSchema>::_schemars_private_is_option())
                                    },
                                    { generator.subschema_for::<String>() },
                                );
                            }
                            {
                                schemars::_private::insert_object_property(
                                    &mut schema,
                                    "description",
                                    if generator.contract().is_serialize() {
                                        false
                                    } else {
                                        false
                                            || (!false
                                                && <Option<
                                                    String,
                                                > as schemars::JsonSchema>::_schemars_private_is_option())
                                    },
                                    { generator.subschema_for::<Option<String>>() },
                                );
                            }
                            schema
                        }
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                impl apistos::ApiComponent for Todo {
                    fn child_schemas(
                        oas_version: apistos::OpenApiVersion,
                    ) -> Vec<
                        (
                            String,
                            apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                        ),
                    > {
                        let settings = oas_version.get_schema_settings();
                        let mut gen = settings.into_generator();
                        let mut schema: apistos::Schema = gen
                            .into_root_schema_for::<Self>();
                        let mut schemas: Vec<
                            (
                                String,
                                apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                            ),
                        > = ::alloc::vec::Vec::new();
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
                                    {
                                        let lvl = ::log::Level::Warn;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!("Error generating json schema: {0:?}", err),
                                                lvl,
                                                &(
                                                    "simple_todo_macros::api::test::todo",
                                                    "simple_todo_macros::api::test::todo",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                    err
                                })
                                .map(|sch| apistos::ApistosSchema::new(sch, oas_version))
                                .unwrap_or_default();
                            schemas
                                .push((
                                    def_name,
                                    apistos::reference_or::ReferenceOr::Object(schema),
                                ));
                        }
                        schemas
                    }
                    fn schema(
                        oas_version: apistos::OpenApiVersion,
                    ) -> Option<
                        (
                            String,
                            apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                        ),
                    > {
                        let (name, schema) = {
                            let schema_name = <Self as schemars::JsonSchema>::schema_name();
                            let settings = oas_version.get_schema_settings();
                            let mut gen = settings.into_generator();
                            let mut schema: apistos::Schema = gen
                                .into_root_schema_for::<Self>();
                            let obj = schema.ensure_object();
                            let schema = apistos::ApistosSchema::new(
                                apistos::Schema::from(obj.clone()),
                                oas_version,
                            );
                            (
                                schema_name,
                                apistos::reference_or::ReferenceOr::Object(schema),
                            )
                        };
                        Some((name.to_string(), schema))
                    }
                }
            };
            #[automatically_derived]
            impl apistos::PathItemDefinition for get_todo {
                fn is_visible() -> bool {
                    true
                }
                fn operation(
                    oas_version: apistos::OpenApiVersion,
                ) -> apistos::paths::Operation {
                    use apistos::ApiComponent;
                    let mut operation_builder = apistos::paths::Operation::default();
                    let mut body_requests: Vec<
                        std::option::Option<apistos::paths::RequestBody>,
                    > = ::alloc::vec::Vec::new();
                    let mut request_body = <Path<Uuid>>::request_body(oas_version);
                    let consumes: Option<String> = None;
                    if let Some(consumes) = consumes {
                        request_body
                            .as_mut()
                            .map(|t| {
                                t.content = t
                                    .content
                                    .values()
                                    .map(|v| (consumes.clone(), v.clone()))
                                    .collect::<
                                        std::collections::BTreeMap<
                                            String,
                                            apistos::paths::MediaType,
                                        >,
                                    >();
                            });
                    }
                    body_requests.push(request_body);
                    let body_requests = body_requests
                        .into_iter()
                        .flatten()
                        .collect::<Vec<apistos::paths::RequestBody>>();
                    for body_request in body_requests {
                        operation_builder.request_body = Some(
                            apistos::reference_or::ReferenceOr::Object(body_request),
                        );
                    }
                    let mut parameters = ::alloc::vec::Vec::new();
                    parameters.append(&mut <Path<Uuid>>::parameters(oas_version));
                    if !parameters.is_empty() {
                        operation_builder.parameters = parameters
                            .into_iter()
                            .map(apistos::reference_or::ReferenceOr::Object)
                            .collect();
                    }
                    if let Some(responses) = <apistos::actix::ResponseWrapper<
                        Box<
                            dyn std::future::Future<
                                Output = Result<Json<Todo>, Error>,
                            > + std::marker::Unpin,
                        >,
                        get_todo,
                    >>::responses(oas_version, None) {
                        operation_builder.responses = responses;
                    }
                    let securities = {
                        let mut needs_empty_security = false;
                        let mut securities = ::alloc::vec::Vec::new();
                        let needed_scopes: std::collections::BTreeMap<
                            String,
                            Vec<String>,
                        > = Default::default();
                        if !<Path<Uuid>>::required() {
                            needs_empty_security = true;
                        }
                        let mut security_requirements = ::alloc::vec::Vec::new();
                        if let Some(security_requirement_name) = <Path<
                            Uuid,
                        >>::security_requirement_name() {
                            let scopes: Vec<String> = needed_scopes
                                .get(&security_requirement_name)
                                .cloned()
                                .unwrap_or_default();
                            security_requirements
                                .push(apistos::security::SecurityRequirement {
                                    requirements: std::collections::BTreeMap::from_iter(
                                        <[_]>::into_vec(
                                            #[rustc_box]
                                            ::alloc::boxed::Box::new([
                                                (security_requirement_name, scopes),
                                            ]),
                                        ),
                                    ),
                                });
                        }
                        securities.append(&mut security_requirements);
                        if needs_empty_security {
                            securities
                                .push(apistos::security::SecurityRequirement::default());
                        }
                        securities
                    };
                    if !securities.is_empty() {
                        operation_builder.security = securities;
                    }
                    operation_builder.operation_id = None;
                    operation_builder.deprecated = Some(false);
                    operation_builder.summary = Some(
                        "Get an element from the todo list".to_string(),
                    );
                    operation_builder
                }
                fn components(
                    oas_version: apistos::OpenApiVersion,
                ) -> Vec<apistos::components::Components> {
                    use apistos::ApiComponent;
                    let mut component_builder = apistos::components::Components::default();
                    for (name, security) in <Path<Uuid>>::securities() {
                        component_builder
                            .security_schemes
                            .insert(
                                name,
                                apistos::reference_or::ReferenceOr::Object(security),
                            );
                    }
                    let mut schemas = ::alloc::vec::Vec::new();
                    let mut schemas = if oas_version == apistos::OpenApiVersion::OAS3_0 {
                        schemas.push(<Path<Uuid>>::schema(oas_version));
                        schemas
                            .push(
                                <apistos::actix::ResponseWrapper<
                                    Box<
                                        dyn std::future::Future<
                                            Output = Result<Json<Todo>, Error>,
                                        > + std::marker::Unpin,
                                    >,
                                    get_todo,
                                >>::schema(oas_version),
                            );
                        schemas
                            .into_iter()
                            .flatten()
                            .collect::<
                                Vec<
                                    (
                                        String,
                                        apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                                    ),
                                >,
                            >()
                    } else {
                        ::alloc::vec::Vec::new()
                    };
                    schemas.append(&mut <Path<Uuid>>::child_schemas(oas_version));
                    schemas
                        .append(
                            &mut <apistos::actix::ResponseWrapper<
                                Box<
                                    dyn std::future::Future<
                                        Output = Result<Json<Todo>, Error>,
                                    > + std::marker::Unpin,
                                >,
                                get_todo,
                            >>::child_schemas(oas_version),
                        );
                    let error_schemas = <apistos::actix::ResponseWrapper<
                        Box<
                            dyn std::future::Future<
                                Output = Result<Json<Todo>, Error>,
                            > + std::marker::Unpin,
                        >,
                        get_todo,
                    >>::error_schemas(oas_version);
                    let mut schemas = std::collections::BTreeMap::from_iter(schemas);
                    component_builder.schemas = schemas;
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([component_builder]),
                    )
                }
            }
            impl ::apistos::DefinitionHolder for get_todo {
                fn path(&self) -> &str {
                    "/todo/{todo_id}"
                }
                fn operations(
                    &mut self,
                    oas_version: apistos::OpenApiVersion,
                ) -> apistos::IndexMap<
                    apistos::paths::OperationType,
                    apistos::paths::Operation,
                > {
                    use ::apistos::PathItemDefinition;
                    apistos::IndexMap::from_iter(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                (
                                    apistos::paths::OperationType::Get,
                                    { get_todo::operation(oas_version) },
                                ),
                            ]),
                        ),
                    )
                }
                fn components(
                    &mut self,
                    oas_version: apistos::OpenApiVersion,
                ) -> Vec<apistos::components::Components> {
                    use ::apistos::PathItemDefinition;
                    <get_todo as ::apistos::PathItemDefinition>::components(oas_version)
                }
            }
            #[allow(non_camel_case_types, missing_docs)]
            pub struct get_todo;
            impl ::actix_web::dev::HttpServiceFactory for get_todo {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    pub(crate) async fn get_todo(
                        todo_id: Path<Uuid>,
                    ) -> Result<Json<Todo>, Error> {
                        Ok(
                            Json(Todo {
                                id: todo_id.into_inner(),
                                title: "some title".to_string(),
                                description: None,
                            }),
                        )
                    }
                    let __resource = ::actix_web::Resource::new("/todo/{todo_id}")
                        .name("get_todo")
                        .guard(::actix_web::guard::Get())
                        .to(get_todo);
                    ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
                }
            }
            #[automatically_derived]
            impl apistos::PathItemDefinition for add_todo {
                fn is_visible() -> bool {
                    true
                }
                fn operation(
                    oas_version: apistos::OpenApiVersion,
                ) -> apistos::paths::Operation {
                    use apistos::ApiComponent;
                    let mut operation_builder = apistos::paths::Operation::default();
                    let mut body_requests: Vec<
                        std::option::Option<apistos::paths::RequestBody>,
                    > = ::alloc::vec::Vec::new();
                    let mut request_body = <Json<NewTodo>>::request_body(oas_version);
                    let consumes: Option<String> = None;
                    if let Some(consumes) = consumes {
                        request_body
                            .as_mut()
                            .map(|t| {
                                t.content = t
                                    .content
                                    .values()
                                    .map(|v| (consumes.clone(), v.clone()))
                                    .collect::<
                                        std::collections::BTreeMap<
                                            String,
                                            apistos::paths::MediaType,
                                        >,
                                    >();
                            });
                    }
                    body_requests.push(request_body);
                    let body_requests = body_requests
                        .into_iter()
                        .flatten()
                        .collect::<Vec<apistos::paths::RequestBody>>();
                    for body_request in body_requests {
                        operation_builder.request_body = Some(
                            apistos::reference_or::ReferenceOr::Object(body_request),
                        );
                    }
                    let mut parameters = ::alloc::vec::Vec::new();
                    parameters.append(&mut <Json<NewTodo>>::parameters(oas_version));
                    if !parameters.is_empty() {
                        operation_builder.parameters = parameters
                            .into_iter()
                            .map(apistos::reference_or::ReferenceOr::Object)
                            .collect();
                    }
                    if let Some(responses) = <apistos::actix::ResponseWrapper<
                        Box<
                            dyn std::future::Future<
                                Output = Result<Json<Todo>, Error>,
                            > + std::marker::Unpin,
                        >,
                        add_todo,
                    >>::responses(oas_version, None) {
                        operation_builder.responses = responses;
                    }
                    let securities = {
                        let mut needs_empty_security = false;
                        let mut securities = ::alloc::vec::Vec::new();
                        let needed_scopes: std::collections::BTreeMap<
                            String,
                            Vec<String>,
                        > = Default::default();
                        if !<Json<NewTodo>>::required() {
                            needs_empty_security = true;
                        }
                        let mut security_requirements = ::alloc::vec::Vec::new();
                        if let Some(security_requirement_name) = <Json<
                            NewTodo,
                        >>::security_requirement_name() {
                            let scopes: Vec<String> = needed_scopes
                                .get(&security_requirement_name)
                                .cloned()
                                .unwrap_or_default();
                            security_requirements
                                .push(apistos::security::SecurityRequirement {
                                    requirements: std::collections::BTreeMap::from_iter(
                                        <[_]>::into_vec(
                                            #[rustc_box]
                                            ::alloc::boxed::Box::new([
                                                (security_requirement_name, scopes),
                                            ]),
                                        ),
                                    ),
                                });
                        }
                        securities.append(&mut security_requirements);
                        if needs_empty_security {
                            securities
                                .push(apistos::security::SecurityRequirement::default());
                        }
                        securities
                    };
                    if !securities.is_empty() {
                        operation_builder.security = securities;
                    }
                    operation_builder.operation_id = None;
                    operation_builder.deprecated = Some(false);
                    operation_builder.summary = Some(
                        "Add a new element to the todo list".to_string(),
                    );
                    operation_builder
                }
                fn components(
                    oas_version: apistos::OpenApiVersion,
                ) -> Vec<apistos::components::Components> {
                    use apistos::ApiComponent;
                    let mut component_builder = apistos::components::Components::default();
                    for (name, security) in <Json<NewTodo>>::securities() {
                        component_builder
                            .security_schemes
                            .insert(
                                name,
                                apistos::reference_or::ReferenceOr::Object(security),
                            );
                    }
                    let mut schemas = ::alloc::vec::Vec::new();
                    let mut schemas = if oas_version == apistos::OpenApiVersion::OAS3_0 {
                        schemas.push(<Json<NewTodo>>::schema(oas_version));
                        schemas
                            .push(
                                <apistos::actix::ResponseWrapper<
                                    Box<
                                        dyn std::future::Future<
                                            Output = Result<Json<Todo>, Error>,
                                        > + std::marker::Unpin,
                                    >,
                                    add_todo,
                                >>::schema(oas_version),
                            );
                        schemas
                            .into_iter()
                            .flatten()
                            .collect::<
                                Vec<
                                    (
                                        String,
                                        apistos::reference_or::ReferenceOr<apistos::ApistosSchema>,
                                    ),
                                >,
                            >()
                    } else {
                        ::alloc::vec::Vec::new()
                    };
                    schemas.append(&mut <Json<NewTodo>>::child_schemas(oas_version));
                    schemas
                        .append(
                            &mut <apistos::actix::ResponseWrapper<
                                Box<
                                    dyn std::future::Future<
                                        Output = Result<Json<Todo>, Error>,
                                    > + std::marker::Unpin,
                                >,
                                add_todo,
                            >>::child_schemas(oas_version),
                        );
                    let error_schemas = <apistos::actix::ResponseWrapper<
                        Box<
                            dyn std::future::Future<
                                Output = Result<Json<Todo>, Error>,
                            > + std::marker::Unpin,
                        >,
                        add_todo,
                    >>::error_schemas(oas_version);
                    let mut schemas = std::collections::BTreeMap::from_iter(schemas);
                    component_builder.schemas = schemas;
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([component_builder]),
                    )
                }
            }
            impl ::apistos::DefinitionHolder for add_todo {
                fn path(&self) -> &str {
                    "/todo/"
                }
                fn operations(
                    &mut self,
                    oas_version: apistos::OpenApiVersion,
                ) -> apistos::IndexMap<
                    apistos::paths::OperationType,
                    apistos::paths::Operation,
                > {
                    use ::apistos::PathItemDefinition;
                    apistos::IndexMap::from_iter(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                (
                                    apistos::paths::OperationType::Post,
                                    { add_todo::operation(oas_version) },
                                ),
                            ]),
                        ),
                    )
                }
                fn components(
                    &mut self,
                    oas_version: apistos::OpenApiVersion,
                ) -> Vec<apistos::components::Components> {
                    use ::apistos::PathItemDefinition;
                    <add_todo as ::apistos::PathItemDefinition>::components(oas_version)
                }
            }
            #[allow(non_camel_case_types, missing_docs)]
            pub struct add_todo;
            impl ::actix_web::dev::HttpServiceFactory for add_todo {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    pub(crate) async fn add_todo(
                        body: Json<NewTodo>,
                    ) -> Result<Json<Todo>, Error> {
                        let new_todo = body.into_inner();
                        Ok(
                            Json(Todo {
                                id: Uuid::new_v4(),
                                title: new_todo.title,
                                description: new_todo.description,
                            }),
                        )
                    }
                    let __resource = ::actix_web::Resource::new("/todo/")
                        .name("add_todo")
                        .guard(::actix_web::guard::Post())
                        .to(add_todo);
                    ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
                }
            }
        }
    }
}
fn main() -> Result<(), impl Error> {
    <::actix_web::rt::System>::new()
        .block_on(async move {
            {
                HttpServer::new(move || {
                        let spec = Spec {
                            openapi: OpenApiVersion::OAS3_1,
                            info: Info {
                                title: "A well documented API".to_string(),
                                description: Some(
                                    "This is an API documented using Apistos,\na wonderful new tool to document your actix API !"
                                        .to_string(),
                                ),
                                ..Default::default()
                            },
                            servers: <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    Server {
                                        url: "/api/v3".to_string(),
                                        ..Default::default()
                                    },
                                ]),
                            ),
                            ..Default::default()
                        };
                        App::new()
                            .document(spec)
                            .wrap(Logger::default())
                            .service(get_todo)
                            .build_with(
                                "/openapi.json",
                                BuildConfig::default()
                                    .with(RapidocConfig::new(&"/rapidoc"))
                                    .with(RedocConfig::new(&"/redoc"))
                                    .with(ScalarConfig::new(&"/scalar"))
                                    .with(SwaggerUIConfig::new(&"/swagger")),
                            )
                    })
                    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
                    .run()
                    .await
            }
        })
}
