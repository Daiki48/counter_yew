use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

pub struct App {
    value: i32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
            Msg::Reset => {
                self.value = 0;
                console::log!("reset!!");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Counter by yew"}</h1>
                <div class="panel">
                    <button class="btn" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>
                    <button class="btn" onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>
                </div>
                <div class="reset">
                    <button class="btn" onclick={ctx.link().callback(|_| Msg::Reset)}>
                        { "Reset" }
                    </button>
                </div>
                <p class="counter">
                    { self.value }
                </p>
                <p class="footer">
                    { String::from(Date::new_0().to_string())}
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
