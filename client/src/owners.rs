use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Owner {
    id: i32,
    first_name: String,
    last_name: String,
    address: String,
    city: String,
    telephone: String,
}

#[derive(Properties, PartialEq)]
pub struct OwnersListProps {
    pub owners: Vec<Owner>,
}

#[function_component(OwnersList)]
pub fn owners_list(OwnersListProps { owners }: &OwnersListProps) -> Html {
    // owners.iter().map(|owner| html! {
    //     <p>{format!("{}: {}", owner.first_name, owner.last_name)}</p>
    // }).collect()
    html! {
        <table id="owners">
        <thead>
        <tr>
          <th>{"Name"}</th>
          <th>{"Address"}</th>
          <th>{"City"}</th>
          <th>{"Telephone"}</th>
        </tr>
        </thead>
        <tbody>
        {
            owners.iter().map(|owner| html! {
            <tr>
                <td>{format!("{} {}", owner.first_name, owner.last_name)}</td>
                <td>{owner.address.to_owned()}</td>
                <td>{owner.city.to_owned()}</td>
                <td>{owner.telephone.to_owned()}</td>
            </tr>
            }).collect::<Html>()
        }
        </tbody>
      </table>
    }
}
