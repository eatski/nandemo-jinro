use std::iter::repeat;

use atoms::{loading, Heading2, HeadingDescription, unexpected_error};
use firestore_hooks::{use_collection, DataFetchState};
use layouting::{BodyItems, BottomOperaton};
use model::{MemberJSON, RoomEditAction};
use yew::{function_component, html, Html, Properties};
use use_historical::use_historical_read;
use use_can_roll::{use_can_roll_validation,ValidationError};

use crate::common::RollButton;
use crate::use_roll::use_roll;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

fn icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7">
            <path fill-rule="evenodd" d="M7.5 6a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM3.751 20.105a8.25 8.25 0 0116.498 0 .75.75 0 01-.437.695A18.683 18.683 0 0112 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 01-.437-.695z" clip-rule="evenodd" />
        </svg>
    }
}

#[function_component(RollContainer)]
pub fn roll(props: &Props) -> Html {
    let room = use_historical_read::<RoomEditAction>(props.room_id.clone());
    let members = use_collection::<MemberJSON>(&props.room_id);
    let roll = use_roll(props.room_id.as_str());
    let validate = use_can_roll_validation(&props.room_id);
    let state = members.merge(room).merge(validate);
    match state {
        DataFetchState::Loading => loading(),
        DataFetchState::Loaded(((members,room), validation)) => {
            let rule = room.latest.rule.unwrap();
            html! {
                <section>
                    <BodyItems>
                        <Heading2>{"役職を配布します"}</Heading2>
                        <HeadingDescription>{format!("参加者:{} / 役職:{}",members.len(),rule.roles.iter().map(|role| role.number).sum::<usize>())}</HeadingDescription>
                        <div class="w-80 mx-auto mt-12">
                            <ul class="flex flex-col gap-3 mt-4">
                                {
                                    for rule.roles.iter().map(|roll| {
                                        html! {
                                            <li class="flex text-word pb-1 border-solid border-b border-separator">
                                                <span class="text-lg mr-3 grow">
                                                    {roll.name.as_str()}
                                                </span>
                                                <span class="text-word-2nd flex">
                                                    {
                                                        if roll.number > 5 {
                                                            html! {
                                                                <>
                                                                    {icon()}
                                                                    <span class="ml-1 text-lg">
                                                                        {"×"}
                                                                        {roll.number}
                                                                    </span>

                                                                </>

                                                            }
                                                        } else {
                                                            html!{for repeat(icon()).take(roll.number)}
                                                        }
                                                    }
                                                </span>
                                            </li>
                                        }
                                    })
                                }
                            </ul>
                        </div>
                       
                    </BodyItems>
                    
                    {
                        if !validation.is_empty() {
                            let error_to_message = |error: &ValidationError| {
                                match error {
                                    ValidationError::NotEnoughMembers => "2人以上の参加者が必要です",
                                    ValidationError::NotEnoughRoles => "参加者に対して割り振られる役職が足りません",
                                    ValidationError::NoRules => "ルールがまだ未決定です",
                                    ValidationError::RoomOpen => "部屋が締め切られていません",
                                }
                            };
                            html! {
                                <div class="w-full">
                                    {for validation.iter().map(|error| html!{<p class="m-auto w-fit text-error font-bold">{error_to_message(error)}</p>})}
                                </div>
                            }
                        } else {
                            match roll {
                                Some(roll) => {
                                    html! {
                                        <BottomOperaton>
                                            <RollButton onclick={roll}>
                                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                                                        <path fill-rule="evenodd" d="M19.916 4.626a.75.75 0 01.208 1.04l-9 13.5a.75.75 0 01-1.154.114l-6-6a.75.75 0 011.06-1.06l5.353 5.353 8.493-12.739a.75.75 0 011.04-.208z" clip-rule="evenodd" />
                                                    </svg>
                                            </RollButton>
                                        </BottomOperaton>
                                    }
                                },
                                None => loading(),
                            }
                        }
                    }
                </section>
            }
        },
        DataFetchState::Error => {
            unexpected_error()
        }
    }
}
