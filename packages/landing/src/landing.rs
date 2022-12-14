use atoms::{loading, Heading2, HeadingDescription};
use user_id_storage::save_user_id;
use yew::{function_component, html, use_state, Callback, Children, Properties, Html};
use yew_router::hooks::use_navigator;

use model::{self, MemberInput};
use router::Route;
use names::{Generator, Name};

use crate::common::{title, JoinForm};

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children,
}

#[function_component(Card)]
fn card(props: &ChildrenOnlyProps) -> Html {
    html! {
        <div class="h-full rounded-md p-3 transition-colors bg-layer">
            {props.children.clone()}
        </div>
    }
}

#[function_component(CardContent)]
pub fn card_content(props: &ChildrenOnlyProps) -> Html {
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
                    // <section class="w-full max-w-xl" aria-label="テンプレから">
                    //     <Card>
                    //         <Heading2>{"テンプレから"}</Heading2>
                    //         <HeadingDescription>{"誰かが作ったルールで遊ぶ"}</HeadingDescription>
                    //         <CardContent>
                    //             <div class="flex flex-wrap justify-center gap-2">
                    //                 {button_link("スプラトゥーン", "/tags/spatoon")}
                    //                 {button_link("汎用 4人", "/tags/monster_hunter")}
                    //             </div>
                    //         </CardContent>
                    //     </Card>
                    // </section>
                </div>
                <section>
                    <Heading2>{"遊び方"}</Heading2>
                    <div  class="w-full flex justify-center ml-4 md:ml-0 ">
                        <ol class="text-word-2nd text-sm list-decimal space-y-1">
                            <li>{"みんなに部屋のURLを共有"}</li>
                            <li>{"好きな配役（人狼、市民など）を設定"}</li>
                            <li>{"当アプリがランダムな配役をブラウザを通じて各プレイヤーに送信します"}</li>
                            <li>{"配られた秘密の役職で自由に遊ぼう！"}</li>
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
        Error,
    }
    let history = use_navigator().unwrap();
    let state = use_state(|| State::Input);
    match &*state {
        State::Input => {
            html! {
                <JoinForm form_label="名前を入力してルームを作成する" label="作成" default="ホスト" placeholder="あなたの名前" onsubmit={Callback::from(move |name: String| {
                    state.set(State::Loading);
                    let mut generator = Generator::with_naming(Name::Numbered);
                    let state = state.clone();
                    let room_id = generator.next().unwrap();
                    let room_id_cloned = room_id.clone();
                    let navigator = history.clone();
                    let member_id = firestore::add_document(
                        &room_id,
                        &MemberInput {
                            name,
                            is_host: true
                        },
                        move |_| {
                            navigator.push(&Route::Room { id: room_id_cloned});
                        },
                        move || {
                            state.set(State::Error);
                        }
                    );
                    save_user_id(room_id.as_str(),member_id.as_str());
                })}/>
            }
        }
        State::Loading => html! {
            {loading()}
        },
        State::Error => todo!(),
    }
}
