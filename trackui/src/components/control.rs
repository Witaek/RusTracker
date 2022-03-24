use super::map_component::Plane;
use yew::services::ConsoleService;
use yew::{html::ImplicitClone, prelude::*};

pub enum Msg {
    PlaneChosen(Plane),
}

pub struct Control {
    link: ComponentLink<Self>,
    planes: Vec<Plane>,
    props: Props,
}

#[derive(Clone)]
pub struct Planes {
    pub list: Vec<Plane>,
}

impl ImplicitClone for Planes {}

#[derive(Properties, Clone)]
pub struct Props {
    pub planes: Planes,
    pub select_plane: Callback<Plane>,
}

impl Control {
    fn button(&self, plane: Plane) -> Html {
        let callsign = plane.callsign.clone();
        let cb = self.link.callback(move |_| Msg::PlaneChosen(plane.clone()));
        html! {
            <button onclick=cb>{callsign}</button>
        }
    }
}

impl Component for Control {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Control {
            link,
            planes: props.planes.list.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlaneChosen(plane) => {
                ConsoleService::info(format!("Update: {:?}", plane.callsign).as_ref());
                self.props.select_plane.emit(plane);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="control component-container">
                <h1>{"Plane list"}</h1>
                <div>
                    {for self.planes.iter().map(|plane| Self::button(&self, plane.clone()))}
                    </div>

            </div>
        }
    }
}
