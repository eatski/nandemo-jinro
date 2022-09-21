
use presentational::{header, title, CardListContainer,Card,Heading2WithDescription, input_and_button, Main, CardBgType, CardContent,InputAndButton,tag_list, footer, list};
use yew::prelude::*;
use web_sys::console;

mod members;
mod firestore;

use members::Lobby;

struct Model();

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                {header()}
                <Main>
                    {title()}
                    <CardListContainer>
                        <Card bg_type={CardBgType::Colored}>
                            <Heading2WithDescription title="ルール作成から" description="オリジナルのルールで遊ぶ"/>
                            <CardContent>
                                <InputAndButton label="作成" placeholder="あなたの名前" onsubmit={Callback::from(|val: String| {
                                    console::log_1(&val.into());
                                })}/>
                            </CardContent>
                        </Card>
                        <Card bg_type={CardBgType::Colored}>
                            <Heading2WithDescription title="テンプレから" description="誰かが作ったルールで遊ぶ"/>
                            <CardContent>
                                {tag_list()}
                            </CardContent>
                        </Card>
                    </CardListContainer>
                    {list("遊び方",vec![
                        "好きなルールを選ぶor作成して",
                        "みんなに部屋のURLを共有して",
                        "全員に秘密の役職が配られるからそれを使って自由に遊ぼう！",
                    ])}
                    // <Lobby />
                </Main>
                {footer()}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
