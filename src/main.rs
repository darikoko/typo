use monkeytype::{Game, CharColor};

use web_sys::{Element, EventTarget, HtmlElement, HtmlTextAreaElement, Node};
use yew::prelude::*;
use yew::{function_component, html, props, virtual_dom::AttrValue, Html, Properties};

fn main() {
    yew::Renderer::<App>::new().render();
}

struct GameComponent {
    game: Game,
}

enum Msg {
    CharTyped(String),
    Reload,
}

impl Component for GameComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let game = Game::generate(10);
        Self {
            game: game,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CharTyped(data) => {
                let mut my_char = data.chars().next().expect("String is empty");
                if data == String::from("Backspace") {
                    my_char = '\u{8}';
                }
                self.game.input(my_char);
            },
            Msg::Reload =>{
               self.game = Game::generate(10);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div
                class="frame"
                contenteditable="true"
                spellcheck="false"
                onkeydown={
                    ctx.link().callback(|x:KeyboardEvent| {
                            web_sys::console::log_1(&x.key().into());
                            x.prevent_default();
                            Msg::CharTyped(x.key())
                    })
            }>
            {self.game.characters.iter().enumerate().map(|(index,character)|{
                let mut color_class : String = match character.color{
                    CharColor::White => String::from("white"),
                    CharColor::Red => String::from("red"),
                    CharColor::TransparentRed => String::from("transparent-red"),
                    CharColor::Gray => String::from("gray"),
                };
                if character.underlined{
                    color_class.push_str(" underlined");
                }

                html!{
                    <>
                    <span class={ color_class }>
                        if self.game.position == index{
                            <div class="cursor"></div>
                        }
                        { character.value }
                    </span>
                    </>
                }
            }).collect::<Html>()}
            </div>
            <span
                onclick={
                    ctx.link().callback(|_| {
                            Msg::Reload
                    })}
                >
                { "RELOAD" }
                </span>
            </>
        }
    }
}

// Then supply the prop
#[function_component]
fn App() -> Html {
    let game = Game::new(String::from("remove hide hope"));
    html! {<GameComponent  />}
}
