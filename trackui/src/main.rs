use crate::components::control::{Planes, Control};
use crate::components::map_component::{Plane, MapComponent, Point};
use yew::prelude::*;
mod components;

enum Msg {
    SelectPlane(Plane),
}

struct Model {
    plane: Plane,
    planes: Planes,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let icao1 = Plane {
            callsign: "KLM1023".to_string(),
            lat: Point(52f64, 2f64),
        };
        let icao2 = Plane {
            callsign: "FRA8175".to_string(),
            lat: Point(53f64, 3f64),
        };
        let planes: Planes = Planes {
            list: vec![icao1, icao2],
        };
        let plane = planes.list[0].clone();
        Self { plane, planes, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectPlane(plane) => {
                self.plane = self
                    .planes
                    .list
                    .iter()
                    .find(|c| c.callsign == plane.callsign)
                    .unwrap()
                    .clone();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    //view of the selected city
    fn view(&self) -> Html {
        let cb = self.link.callback(|name| Msg::SelectPlane(name));
        html! {
            <>
                <MapComponent plane=&self.plane planes=&self.planes/>
                <Control select_plane=cb planes=&self.planes/>
            </>
        }
    }
}

fn main() {
    yew::initialize();
    let document = yew::utils::document();
    let app = document.query_selector("#yew").unwrap().unwrap();

    yew::App::<Model>::new().mount(app);

    yew::run_loop();
}
