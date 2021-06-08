use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct App {
    link: ComponentLink<Self>,
    counter: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link,
            counter: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <h1>{ "Welcome to Components" }</h1>
            <p>{"Counter: "} { self.counter } </p>
                <button onclick=self.link.callback(|_| Msg::AddOne)> { "Add 1" } </button>
            </div>
        }
    }
}