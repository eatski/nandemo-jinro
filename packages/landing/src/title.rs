use yew::{Html, html};

pub fn title() -> Html {
    html! {
        <div class="text-center p-8">
            <h1 class="text-2xl md:text-4xl text-black font-mono font-bold">{"なんでも人狼"}</h1>
            <p class="text-black-light mt-2">{"なんでも人狼ゲームにして遊べちゃうブラウザアプリ"}</p>
        </div>
    }
}