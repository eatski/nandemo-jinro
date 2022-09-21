use web_sys::console;
use yew::{function_component, html, Callback};
use presentational::{title, CardListContainer,Card,Heading2WithDescription, Main, CardBgType, CardContent,InputAndButton,tag_list, footer, list};

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <Main>
            {title()}
            <CardListContainer>
                <Card bg_type={CardBgType::Colored}>
                    <Heading2WithDescription title="ルール作成から" description="オリジナルのルールで遊ぶ"/>
                    <CardContent>
                        <CreateRule />
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
    }
}


#[function_component(CreateRule)]
fn create_rule_view() -> Html {
    html! {
        <InputAndButton label="作成" placeholder="あなたの名前" onsubmit={Callback::from(|val: String| {
            console::log_1(&val.into());
        })}/>
    }
}