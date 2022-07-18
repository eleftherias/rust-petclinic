pub mod owners;
pub mod pet;

use owners::OwnersList;
use pet::PetForm;
use reqwasm::http::Request;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/owners")]
    Owners,
    #[to("/owners/<owner_id>/pets/new")]
    Pet { owner_id: i32 },
    #[not_found]
    NotFound,
}

#[component]
async fn Owners<G: Html>(cx: Scope<'_>) -> View<G> {
    let owners: &Signal<Vec<dto::Owner>> = create_signal(cx, Vec::new());
    owners.set(
        Request::get("http://localhost:3000/owners")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap(),
    );

    view! { cx,
        OwnersList{owners : owners}
    }
}

fn main() {
    sycamore::render(|cx| {
        view! {cx,
            Router {
                integration: HistoryIntegration::new(),
                view: |cx, route: &ReadSignal<AppRoutes>| {
                    view! {cx,
                        div(class="app") {
                            ul(class="navbar") {
                                li(class="navbar-item") {
                                    a(href="/") {
                                        "HOME"
                                    }
                                }
                                li(class="navbar-item") {
                                    a(href="/owners") {
                                        "OWNERS"
                                    }
                                }
                            }
                            div(class="main") {
                                (match route.get().as_ref() {
                                    AppRoutes::Home => view! {cx,
                                        h1 {
                                            "Home"
                                        }
                                    },
                                    AppRoutes::Owners => view! {cx,
                                        Owners()
                                    },
                                    AppRoutes::Pet { owner_id } => view! {cx,
                                        PetForm {
                                            owner_id: *owner_id
                                        }
                                    },
                                    AppRoutes::NotFound => view! {cx,
                                        h1 {
                                            "This page does not exist"
                                        }
                                    },
                                })
                            }
                        }
                    }
                }
            }
        }
    });
}
