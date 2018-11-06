
#[macro_use]
extern crate yew;

use yew::prelude::*;
//use stdweb::web::Date;
use yew::services::ConsoleService;

pub struct Model{
    console: ConsoleService,
////    value: String,
}

pub enum Msg{
    Click,
//    Hover,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender{
        match msg {
            Msg::Click =>{
                self.console.log("clicked!");
                true
            }
//            true
        }
    }
}

impl Renderable<Model> for Model{
    fn view(&self) -> Html<Self> {
        html!{
            <div>
                <button onclick=|_| Msg::Click,>{"Click"}</button>
            </div>
        }
    }
}