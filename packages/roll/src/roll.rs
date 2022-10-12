use std::iter::repeat;

use atoms::{loading, Heading2, HeadingDescription, unexpected_error};
use firestore_hooks::{use_collection, use_document, DataFetchState};
use layouting::{BodyItems, BottomOperaton};
use model::{MemberJSON, Room};
use yew::{function_component, html, Callback, Html, Properties};

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
    let room = use_document::<Room>(&(), props.room_id.as_str());
    let members = use_collection::<MemberJSON>(&props.room_id);
    let roll = use_roll(props.room_id.as_str());
    let state = members.merge(room);
    match state {
        DataFetchState::Loading => loading(),
        DataFetchState::Loaded((members, room)) => {
            let rule = room.rule.unwrap();
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
                                            <li class="flex text-black pb-1 border-solid border-b border-line">
                                                <span class="text-lg mr-3 grow">
                                                    {roll.name.as_str()}
                                                </span>
                                                <span class="text-black-light flex">
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
                        match roll {
                            Some(roll) => {
                                html! {
                                    <BottomOperaton>
                                        <RollButton onclick={Callback::once(move |_| roll())}>
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
                    <div class="flex justify-center mt-4">
                        <button class="text-black hover:text-black-light text-md">{"戻る"}</button>
                    </div>
                </section>
            }
        },
        DataFetchState::Error => {
            unexpected_error()
        }
    }
}
