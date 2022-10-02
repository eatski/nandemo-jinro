use yew::{function_component, html, Callback, use_state};
use presentational::{title, CardListContainer,Card,Heading2WithDescription, Main, CardBgType, CardContent,InputAndButton,tag_list, list, loading};
use yew_router::prelude::{use_history, History};

use crate::{router::Route,storage::{save_user_id}};
use model::{self, MemberInput, Room};


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
                "好きなルールを選ぶor作成",
                "みんなに部屋のURLを共有",
                "全員に配られる秘密の役職で自由に遊ぼう！",
            ])}
        </Main>
    }
}


#[function_component(CreateRule)]
fn create_rule_view() -> Html {
    enum State {
        Input,
        Loading,
        Error
    }
    let history = use_history().unwrap();
    let state = use_state(|| State::Input);
    match &*state {
    State::Input => html! {
        <InputAndButton label="作成" default="ホスト" placeholder="あなたの名前" onsubmit={Callback::once(move |name: String| {
            state.set(State::Loading);
            firestore::add_document(
                &(),
                &Room {
                    can_join: true,
                    rule: None,
                },
                move |room_id| {
                let room_id_string = room_id.to_string();
                let member_id = firestore::add_document(
                    &room_id.to_string(), 
                    &MemberInput {
                        name,
                        is_host: true
                    }, 
                    move |_| {
                        history.push(Route::Room { id: room_id_string});
                    },
                    move || {
                        state.set(State::Error);
                    }
                );
                save_user_id(room_id,member_id.as_str());
            },|| {

            });
        })}/>
    },
    State::Loading => html! {
        {loading()}
    },
    State::Error => todo!(),
}
    
}