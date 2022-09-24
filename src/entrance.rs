use firestore::{add_members, MemberInput};
use presentational::{loading, title,SimpleCenteringSection,Heading2,InputAndButton};
use yew::{function_component, Properties, use_state, use_effect_with_deps, Callback, html};


#[derive(Properties, PartialEq)]
pub struct GuestEntranceProps {
    pub room_id: String,
}

#[function_component(GuestEntrance)]
pub fn guest_entrance(props: &GuestEntranceProps) -> Html {
    enum HostNameState {
        Loading,
        Loaded {
            host_name: String,
        },
        Error,
    }
    let state = use_state(|| HostNameState::Loading);
    {
        let state = state.clone();
        let state_on_error = state.clone();
        let room_id = props.room_id.clone();
        use_effect_with_deps(
            |room_id| {
                firestore::get_members(
                    room_id.as_str(),
                    move |members| {
                        let host_name = members
                            .iter()
                            .find(|member| member.is_host)
                            .map(|member| member.name.clone());
                        match host_name {
                            Some(host_name) => state.set(HostNameState::Loaded { host_name }),
                            None => state.set(HostNameState::Error),
                        };
                    },
                    move || {
                        state_on_error.set(HostNameState::Error);
                    },
                );
                || {}
            },
            room_id,
        );
    }
    let room_id = props.room_id.clone();
    let add_member = Callback::from(move |name| {
        let room_id_cloned = room_id.clone();
        let user_id = add_members(
            room_id.as_str(),
            &MemberInput {
                name,
                is_host: false,
            },
            move || {},
            || {}
        );
        crate::storage::save_user_id(room_id_cloned.as_str(),user_id.as_str());
    });

    enum CanJoinState {
        Loading,
        Loaded{
            can_join: bool,
        }
    }

    let can_join_state = use_state(|| CanJoinState::Loading);
    let can_join_state_cloned = can_join_state.clone();
    use_effect_with_deps(move |room_id| {
        let state = can_join_state_cloned.clone();
        firestore::sync_room(room_id, move |room| {
            state.set(CanJoinState::Loaded{can_join: room.can_join})
        }, || {})
    },props.room_id.clone());

    match &*state {
        HostNameState::Loading => loading(),
        HostNameState::Loaded { host_name } => {
            let can_join = match &*can_join_state {
                CanJoinState::Loading => true,
                CanJoinState::Loaded{can_join} => *can_join,
            };
            if can_join {
                html! {
                    <div>
                        {title()}
                        <SimpleCenteringSection>
                            <Heading2>{ format!("「{}」の部屋",host_name)}</Heading2>
                            <InputAndButton label="参加" placeholder="あなたの名前" onsubmit={add_member} />
                        </SimpleCenteringSection>
                    </div>
                }
            }  else {
                html! {
                    <div>
                        {title()}
                        <SimpleCenteringSection>
                            <p>{"この部屋は参加を締め切られました。"}</p>
                        </SimpleCenteringSection>
                    </div>
                }
            }
        },
        HostNameState::Error => todo!(),
    }
}
