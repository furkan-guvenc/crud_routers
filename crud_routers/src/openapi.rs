use std::borrow::Cow;
use utoipa::openapi::request_body::RequestBodyBuilder;
use utoipa::openapi::Tag;
use crate::{ApiServer, Assignable, Assigned, CrudRouterBuilder, Empty, Pagination, ReadDeleteRepository};

impl utoipa::PartialSchema for Empty {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        unimplemented!()
    }
}

impl utoipa::ToSchema for Empty {
    fn name() -> Cow<'static, str> {
        unimplemented!()
    }

    fn schemas(_schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>) {
        unimplemented!()
    }
}

impl<T: utoipa::PartialSchema> utoipa::PartialSchema for Assigned<T> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
        T::schema()
    }
}

impl<T: utoipa::ToSchema> utoipa::ToSchema for Assigned<T> {
    fn name() -> Cow<'static, str> {
        T::name()
    }

    fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>) {
        T::schemas(schemas)
    }
}

impl<Server: ApiServer, Repo: ReadDeleteRepository<Schema, PrimaryKeyType>, Schema: utoipa::ToSchema, PrimaryKeyType, CreateSchema: Assignable + utoipa::ToSchema, UpdateSchema: Assignable + utoipa::ToSchema> CrudRouterBuilder<'_, Assigned<Server>, Repo, Assigned<Schema>, Assigned<PrimaryKeyType>, CreateSchema, UpdateSchema> {
    pub fn build_openapi(self, openapi: &mut utoipa::openapi::OpenApi) -> Self {
        let table_name = Repo::get_table_name();
        let tag = self.tag.or(Some(&table_name)).unwrap();
        let prefix = self.get_prefix();
        let path = Server::get_path(&prefix);
        let id_path = format!("/{}/{{id}}", &prefix);
        let mut openapi_paths = utoipa::openapi::path::Paths::new();
        let mut openapi_schemas = Vec::<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>::new();

        let id_parameter = utoipa::openapi::path::ParameterBuilder::from(utoipa::openapi::path::Parameter::new("id"))
            .parameter_in(utoipa::openapi::path::ParameterIn::Path)
            .description(Some(format!("{} id", table_name)))
            .schema(Some(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
            ))
            .required(utoipa::openapi::Required::True)
            .build();

        let single_item_ref = utoipa::openapi::schema::RefBuilder::new()
            .ref_location_from_schema_name(<Schema as utoipa::ToSchema>::name())
            .build();
        let single_item_response = utoipa::openapi::content::ContentBuilder::new()
            .schema(Some(
                single_item_ref.clone()
            )).build();

        if !self.list_items_route_disabled {
            let list_of_items_response = utoipa::openapi::content::ContentBuilder::new()
                .schema(Some(
                    utoipa::openapi::schema::ArrayBuilder::new()
                        .items(single_item_ref.clone()))
                ).build();

            openapi_paths.add_path_operation(
                &path,
                vec![utoipa::openapi::HttpMethod::Get],
                utoipa::openapi::path::OperationBuilder::new()
                    .tag(tag)
                    .description(Some(format!("Lists all {}", table_name)))
                    .operation_id(Some(format!("list_all_{}", table_name)))
                    .parameters(Some(<Pagination as utoipa::IntoParams>::into_params(|| Some(utoipa::openapi::path::ParameterIn::Query))))
                    .response(
                        "200",
                        utoipa::openapi::ResponseBuilder::new()
                            .description(format!("All {} listed successfully", table_name))
                            .content("application/json", list_of_items_response)
                            .build()
                    )
            );
            openapi_schemas.push((<Schema as utoipa::ToSchema>::name().to_string(), <Schema as utoipa::PartialSchema>::schema()));
            <Schema as utoipa::ToSchema>::schemas(&mut openapi_schemas);
        }

        if !self.get_item_route_disabled {
            let optional_item_response = utoipa::openapi::content::ContentBuilder::new()
                .schema(Some(utoipa::openapi::schema::OneOfBuilder::new()
                    .item(
                        utoipa::openapi::schema::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::Type::Null)
                    )
                    .item(single_item_ref))).build();

            openapi_paths.add_path_operation(
                &id_path,
                vec![utoipa::openapi::HttpMethod::Get],
                utoipa::openapi::path::OperationBuilder::new()
                    .tag(tag)
                    .description(Some(format!("Gets one {}", table_name)))
                    .operation_id(Some(format!("get_{}", table_name)))
                    .parameter(id_parameter.clone())
                    .response(
                        "200",
                        utoipa::openapi::ResponseBuilder::new()
                            .description(format!("One {} is fetched successfully", table_name))
                            .content(
                                "application/json", optional_item_response
                            ).build()
                    )
            );
            openapi_schemas.push((<Schema as utoipa::ToSchema>::name().to_string(), <Schema as utoipa::PartialSchema>::schema()));
            <Schema as utoipa::ToSchema>::schemas(&mut openapi_schemas);
        }

        if !self.delete_all_items_route_disabled {
            let integer_response = utoipa::openapi::content::ContentBuilder::new()
                .schema(Some(
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                        .minimum(Some(0f64))
                )).build();

            openapi_paths.add_path_operation(
                &path,
                vec![utoipa::openapi::HttpMethod::Delete],
                utoipa::openapi::path::OperationBuilder::new()
                    .tag(tag)
                    .description(Some(format!("Deletes all {}", table_name)))
                    .operation_id(Some(format!("delete_all_{}", table_name)))
                    .response(
                        "200",
                        utoipa::openapi::ResponseBuilder::new()
                            .description(format!("All {} deleted successfully", table_name))
                            .content("text/plain",integer_response).build()
                    )
            );
        }

        if !self.delete_item_route_disabled {
            openapi_paths.add_path_operation(
                &id_path,
                vec![utoipa::openapi::HttpMethod::Delete],
                utoipa::openapi::path::OperationBuilder::new()
                    .tag(tag)
                    .description(Some(format!("Deletes one {}", table_name)))
                    .operation_id(Some(format!("delete_{}", table_name)))
                    .parameter(id_parameter.clone())
                    .response(
                        "200",
                        utoipa::openapi::ResponseBuilder::new()
                            .description(format!("One {} is deleted successfully", table_name))
                            .build()
                    )
            );
        }

        if !self.create_item_route_disabled && CreateSchema::IS_ASSIGNED {
            let create_item_request = utoipa::openapi::content::ContentBuilder::new()
                .schema(Some(
                    utoipa::openapi::schema::RefBuilder::new()
                        .ref_location_from_schema_name(<CreateSchema as utoipa::ToSchema>::name())
                        .build()
                )).build();

            openapi_paths.add_path_operation(
                &path,
                vec![utoipa::openapi::HttpMethod::Post],
                utoipa::openapi::path::OperationBuilder::new()
                    .tag(tag)
                    .description(Some(format!("Creates {}", table_name)))
                    .operation_id(Some(format!("create_{}", table_name)))
                    .request_body(Some(
                        RequestBodyBuilder::new()
                            .content("application/json", create_item_request)
                            .required(Some(utoipa::openapi::Required::True))
                            .build()
                    ))
                    .response(
                        "200",
                        utoipa::openapi::ResponseBuilder::new()
                            .description(format!("One {} is created successfully", table_name))
                            .content(
                                "application/json",
                                single_item_response.clone()
                            )
                            .build()
                    )
            );
            openapi_schemas.push((<Schema as utoipa::ToSchema>::name().to_string(), <Schema as utoipa::PartialSchema>::schema()));
            openapi_schemas.push((<CreateSchema as utoipa::ToSchema>::name().to_string(), <CreateSchema as utoipa::PartialSchema>::schema()));
            <Schema as utoipa::ToSchema>::schemas(&mut openapi_schemas);
            <CreateSchema as utoipa::ToSchema>::schemas(&mut openapi_schemas);
        }

        if !self.update_item_route_disabled && UpdateSchema::IS_ASSIGNED {
            let update_item_request = utoipa::openapi::content::ContentBuilder::new()
                .schema(Some(
                    utoipa::openapi::schema::RefBuilder::new()
                        .ref_location_from_schema_name(<UpdateSchema as utoipa::ToSchema>::name())
                        .build()
                )).build();

            openapi_paths.add_path_operation(
                &id_path,
                vec![utoipa::openapi::HttpMethod::Put],
                utoipa::openapi::path::OperationBuilder::new()
                    .tag(tag)
                    .description(Some(format!("Updates {}", table_name)))
                    .operation_id(Some(format!("update_{}", table_name)))
                    .parameter(id_parameter)
                    .request_body(Some(
                        RequestBodyBuilder::new()
                            .content("application/json", update_item_request)
                            .required(Some(utoipa::openapi::Required::True))
                            .build()
                    ))
                    .response(
                        "200",
                        utoipa::openapi::ResponseBuilder::new()
                            .description(format!("One {} is updated successfully", table_name))
                            .content(
                                "application/json",
                                single_item_response
                            )
                            .build()
                    )
            );
            openapi_schemas.push((<Schema as utoipa::ToSchema>::name().to_string(), <Schema as utoipa::PartialSchema>::schema()));
            openapi_schemas.push((<UpdateSchema as utoipa::ToSchema>::name().to_string(), <UpdateSchema as utoipa::PartialSchema>::schema()));
            <Schema as utoipa::ToSchema>::schemas(&mut openapi_schemas);
            <UpdateSchema as utoipa::ToSchema>::schemas(&mut openapi_schemas);
        }

        openapi.paths.paths.extend(openapi_paths.paths);
        let tags = openapi
            .tags
            .get_or_insert(vec![]);
        tags.push(Tag::new(tag));
        let components = openapi
            .components
            .get_or_insert(utoipa::openapi::Components::new());
        components.schemas.extend(openapi_schemas);

        self
    }
}
