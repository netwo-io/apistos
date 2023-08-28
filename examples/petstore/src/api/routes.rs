use crate::api::handlers::{add_pet, find_by_status, find_by_tags, get_pet, update_pet, update_pet_with_form};
use netwopenapi::web::{delete, get, post, put, resource, scope, Scope};

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
      .service(resource("/findByTags").route(get().to(find_by_tags)))
      .service(
        scope("/{petId}").service(
          resource("")
            .route(get().to(get_pet))
          .route(post().to(update_pet_with_form))
          // .route(delete().to(test)),
        ), // .service(resource("/uploadImage").route(post().to(test))),
      ),
  )
  // .service(
  //   scope("/store")
  //     .service(
  //       scope("")
  //         .service(resource("/inventory").route(get().to(test)))
  //         .service(resource("/order").route(post().to(test)))
  //     )
  //     .service(
  //       scope("/order").service(resource("/{orderId}")
  //         .route(get().to(test))
  //         .route(delete().to(test))
  //       )
  //     )
  // )
  // .service(
  //   scope("/user")
  //     .service(resource("").route(post().to(test)))
  //     .service(resource("/createWithArray").route(post().to(test)))
  //     .service(resource("/createWithList").route(post().to(test)))
  //     .service(resource("/login").route(get().to(test)))
  //     .service(resource("/logout").route(get().to(test)))
  //     .service(
  //       resource("/{username}")
  //         .route(get().to(test))
  //         .route(put().to(test))
  //         .route(delete().to(test))
  //     )
  // )
}
