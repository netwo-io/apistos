#[allow(deprecated)]
use crate::api::handlers::{
  add_pet, delete_pet, find_by_status, find_by_tags, get_pet, update_pet, update_pet_with_form,
};
use apistos::web::{delete, get, post, put, resource, scope, Scope};

#[rustfmt::skip]
pub(crate) fn routes() -> Scope {
  scope("").service(
    scope("/pet")
      .service(
        resource("")
          .route(post().to(add_pet))
          .route(put().to(update_pet))
      )
      .service(resource("/findByStatus").route(get().to(find_by_status)))
      .service(resource("/findByTags").route(get().to(#[allow(deprecated)] find_by_tags)))
      .service(
        scope("/{petId:.+}").service(
          resource("")
            .route(get().to(get_pet))
            .route(post().to(update_pet_with_form))
            .route(delete().to(delete_pet)),
        )
      ),
  )
}
