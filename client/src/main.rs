pub mod owners;

use owners::Owner;
use owners::OwnersList;
use reqwasm::http::Request;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/owners")]
    Owners,
    #[not_found]
    NotFound,
}

#[component(Owners<G>)]
fn all_owners() -> View<G> {
    let owners: Signal<Vec<Owner>> = Signal::new(vec![]);
    {
        let owners = owners.clone();
        create_effect(move || {
            let owners = owners.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_owners: Vec<Owner> = Request::get("http://localhost:3000/owners")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                owners.set(fetched_owners);
            });
        });
    }

    view! {
        OwnersList(owners.handle())
    }
}

fn main() {
    sycamore::render(|| {
        view! {
            Router(RouterProps::new(HistoryIntegration::new(), |route: ReadSignal<AppRoutes>| {
                let t = create_memo(move || match route.get().as_ref() {
                    AppRoutes::Home => view! {
                        h1 {
                            "Home"
                        }
                    },
                    AppRoutes::Owners => view! {
                        Owners()
                    },
                    AppRoutes::NotFound => view! {
                        h1 {
                            "This page does not exist"
                        }
                    },
                });
                view! {
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
                        (t.get().as_ref().clone())
                    }
                }
            }))
        }
    });
}
