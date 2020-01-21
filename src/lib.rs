#![recursion_limit = "128"]

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use std::borrow::Cow;

pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    steps: Vec<usize>,
}

pub enum Msg {
    StepPress(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            console: ConsoleService::new(),
            steps: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StepPress(step) => {
                let output = Cow::Owned(format!("step {} pressed", step));
                self.console.log(&output);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="tile">
                    <div class="tile is-parent is-vertical">
                        <div class="buttons">
                            {for self.steps.iter().enumerate().map(|step|
                                self.show_step(step)
                            )}
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn show_step(&self, (step, _entry): (usize, &usize)) -> Html {
        html! {
            <button
                class="button is-dark"
                onclick=self.link.callback(move |_| Msg::StepPress(step))
            >
                { step }
            </button>
        }
    }
}
