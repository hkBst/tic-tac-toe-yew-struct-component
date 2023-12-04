use tic_tac_toe::*;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

struct App(Game);

enum Msg {
    Move(FieldName),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self(Game::new())
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Move(field) => self.0.act(field),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let board = [Hor::Left, Hor::Mid, Hor::Right].into_iter().flat_map(|h| {
            [Vert::Top, Vert::Mid, Vert::Bottom]
                .into_iter()
                .map(move |v| {
                    let name = FieldName { v, h };
                    view_field(
                        self.0.get(name),
                        ctx.link().callback(move |_| Msg::Move(name)),
                    )
                })
        });

        html! { <div class="game">
            <div class="board"> {board.collect::<Vec<_>>()} </div>
            <div class="state"> {self.0.state().to_string()} </div>
        </div> }
    }
}

fn view_field(state: FieldState, cb: Callback<MouseEvent>) -> Html {
    let class = match state.0 {
        Some(Side::X) => "cell side-X",
        Some(Side::O) => "cell side-O",
        None => "cell",
    };

    html! { <div class={class} onclick={cb}> {state.to_string()} </div> }
}
