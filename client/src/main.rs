pub mod owners;

use owners::Owner;
use owners::OwnersList;
use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/owners")]
    Owners,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Owners)]
fn all_owners() -> Html {
    let owners = use_state(|| vec![]);
    {
        let owners = owners.clone();
        use_effect_with_deps(
            move |_| {
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
                || ()
            },
            (),
        );
    }
    html! {
        <OwnersList owners={(*owners).clone()} />
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Owners => html! {
            <Owners />
        },
        Route::NotFound => html! { <h1>{ "This page does not exist" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <ul class="navbar">
                <li class="navbar-item">
                    <Link<Route> to={Route::Home}>
                        { "HOME" }
                    </Link<Route>>
                </li>
                <li class="navbar-item">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Owners}>
                        { "OWNERS" }
                    </Link<Route>>
                </li>
            </ul>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
