use user_id_storage::save_user_id;
use yew::{function_component, html, Callback, use_state, Properties, Children};
use atoms::{Heading2,HeadingDescription,loading, button_link};
use yew_router::prelude::{use_history, History};

use router::Route;
use model::{self, MemberInput, Room};

use crate::common::{title,JoinForm};

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children,
}


#[function_component(Card)]
fn card(props:&ChildrenOnlyProps) -> Html {
    html! {
        <div class="h-full rounded-md p-3 bg-colored">
            {props.children.clone()}
        </div>
    }
}

#[function_component(CardContent)]
pub fn card_content(props:&ChildrenOnlyProps) -> Html {
    html! {
        <div class="py-2">
            {props.children.clone()}
        </div>
    }
}


#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <>
            {title()}
            <div class="flex gap-8 flex-col">
                <div class="flex gap-7 flex-col md:flex-row justify-center">
                    <section class="w-full max-w-xl" aria-label="ルール作成から">
                        <Card>
                            <Heading2>{"ルール作成から"}</Heading2>
                            <HeadingDescription>{"オリジナルのルールで遊ぶ"}</HeadingDescription>
                            <CardContent>
                                <CreateRule />
                            </CardContent>
                        </Card>
                    </section>
                    <section class="w-full max-w-xl" aria-label="テンプレから">
                        <Card>
                            <Heading2>{"テンプレから"}</Heading2>
                            <HeadingDescription>{"誰かが作ったルールで遊ぶ"}</HeadingDescription>
                            <CardContent>
                                <div class="flex flex-wrap justify-center gap-2">
                                    {button_link("スプラトゥーン", "/tags/spatoon")}
                                    {button_link("汎用 4人", "/tags/monster_hunter")}
                                </div>
                            </CardContent>
                        </Card>
                    </section>
                    
                </div>
                <section>
                    <Heading2>{"遊び方"}</Heading2>
                    <div  class="w-full flex justify-center">
                        <ol class="text-black-light text-sm list-decimal space-y-1">
                            <li>{"好きなルールを選ぶor作成"}</li>
                            <li>{"みんなに部屋のURLを共有"}</li>
                            <li>{"全員に配られる秘密の役職で自由に遊ぼう！"}</li>
                        </ol>
                    </div>
                </section>
            </div>
        </>
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
    State::Input => {
        html! {
            <JoinForm label="作成" default="ホスト" placeholder="あなたの名前" onsubmit={Callback::once(move |name: String| {
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
        }
    },
    State::Loading => html! {
        {loading()}
    },
    State::Error => todo!(),
}
    
}