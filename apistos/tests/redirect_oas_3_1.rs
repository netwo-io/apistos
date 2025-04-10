use actix_web::App;
use apistos::web::redirect;

use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::OpenApiVersion;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos_models::OpenApi;
use assert_json_diff::assert_json_eq;
use serde_json::json;
#[actix_web::test]
async fn actix_redirect_oas_3_1() {
  let app = App::new()
    .document(Spec {
      openapi: OpenApiVersion::OAS3_1,
      ..Default::default()
    })
    .service(redirect("/duck", "https://duck.com"))
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let json = serde_json::to_value(&body).unwrap();
  assert_json_eq!(
    json,
    json!({
      "openapi": "3.1.0",
      "info": {
        "title": "",
        "version": ""
      },
      "servers": [],
      "paths": {
        "/duck": {
          "get": {
            "operationId": "get_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "put": {
            "operationId": "put_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "post": {
            "operationId": "post_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "delete": {
            "operationId": "delete_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "options": {
            "operationId": "options_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "head": {
            "operationId": "head_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          "patch": {
            "operationId": "patch_duck-1e3015160cf5faf17daf6c059ad0697d",
            "responses": {
              "default": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              },
              "307": {
                "description": "Redirect.",
                "headers": {
                  "Location": {
                    "description": "Redirection URL",
                    "content": {
                      "text/plain": {
                        "schema": {
                          "type": "string",
                          "const": "https://duck.com"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    })
  );
}

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use actix_web_lab as _;
use apistos_core as _;
use apistos_gen as _;
use apistos_plugins as _;
use apistos_rapidoc as _;
use apistos_redoc as _;
use apistos_scalar as _;
use apistos_swagger_ui as _;
use futures_util as _;
use garde_actix_web as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
use schemars as _;
use serde as _;
use serde_json as _;
