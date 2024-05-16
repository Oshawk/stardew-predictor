use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DateJumpProperties {
    pub updated: Callback<i32>,
}

#[function_component]
pub fn DateJump(properties: &DateJumpProperties) -> Html {
    let year: UseStateHandle<Option<u16>> = use_state(|| None);
    let season: UseStateHandle<Option<u8>> = use_state(|| None);
    let day: UseStateHandle<Option<u8>> = use_state(|| None);

    let year_updated: Callback<InputEvent> = {
        let year: UseStateHandle<Option<u16>> = year.clone();
        Callback::from(move |event: InputEvent| {
            match event
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u16>()
            {
                Ok(year_) => {
                    if year_ > 0u16 {
                        year.set(Some(year_));
                    } else {
                        year.set(None);
                    }
                }
                Err(_) => {
                    year.set(None);
                }
            }
        })
    };

    let season_updated: Callback<Event> = {
        let season: UseStateHandle<Option<u8>> = season.clone();
        Callback::from(move |event: Event| {
            let season_: i32 = event
                .target_unchecked_into::<HtmlSelectElement>()
                .selected_index();
            if (1i32..=4i32).contains(&season_) {
                season.set(Some(season_ as u8));
            } else {
                season.set(None);
            }
        })
    };

    let day_updated: Callback<InputEvent> = {
        let day: UseStateHandle<Option<u8>> = day.clone();
        Callback::from(move |event: InputEvent| {
            match event
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
            {
                Ok(day_) => {
                    if (1u8..=28u8).contains(&day_) {
                        day.set(Some(day_));
                    } else {
                        day.set(None);
                    }
                }
                Err(_) => {
                    day.set(None);
                }
            }
        })
    };

    let jump_updated: Callback<MouseEvent> = {
        let updated: Callback<i32> = properties.updated.clone();
        let year: UseStateHandle<Option<u16>> = year.clone();
        let season: UseStateHandle<Option<u8>> = season.clone();
        let day: UseStateHandle<Option<u8>> = day.clone();
        Callback::from(move |_: MouseEvent| match (*year, *season, *day) {
            (Some(year), Some(season), Some(day)) => {
                updated
                    .emit((year as i32 - 1i32) * 112i32 + (season as i32 - 1i32) * 28 + day as i32);
            }
            _ => {}
        })
    };

    html! {
        <div class="field has-addons">
            <div class="control is-expanded">
                <input class="input" oninput={ year_updated } placeholder="Year" type="text" value={ match *year { Some (year) => { year.to_string() }, None => { "".to_string() } } } />
            </div>
            <div class="control">
                <div class="select">
                    <select onchange={ season_updated }>
                        <option disabled=true hidden=true selected={ season.unwrap_or(0u8) == 0u8 }>{ "Season" }</option>
                        <option selected={ season.unwrap_or(0u8) == 1u8 }>{ "Spring" }</option>
                        <option selected={ season.unwrap_or(0u8) == 2u8 }>{ "Summer" }</option>
                        <option selected={ season.unwrap_or(0u8) == 3u8 }>{ "Fall" }</option>
                        <option selected={ season.unwrap_or(0u8) == 4u8 }>{ "Winter" }</option>
                    </select>
                </div>
            </div>
            <div class="control is-expanded">
                <input class="input" oninput={ day_updated } placeholder="Day" type="text" value={ match *day { Some (day) => { day.to_string() }, None => { "".to_string() } } } />
            </div>
            <div class="control">
                <button class="button is-primary" onclick={ jump_updated }>{ "Jump" }</button>
            </div>
        </div>
    }
}
