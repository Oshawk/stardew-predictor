use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

fn compute_date(year: Option<u16>, season: Option<u8>, day: Option<u8>) -> Option<i32> {
    match (year, season, day) {
        (Some(year), Some(season), Some(day)) => {
            Some((year as i32 - 1) * 112 + (season as i32 - 1) * 28 + day as i32)
        }
        _ => None,
    }
}

/// Continuous date picker: emits `Option<i32>` on every field change.
#[derive(Properties, PartialEq)]
pub struct DatePickerProperties {
    pub updated: Callback<Option<i32>>,
    #[prop_or(AttrValue::from("Date"))]
    pub label: AttrValue,
}

#[component]
pub fn DatePicker(properties: &DatePickerProperties) -> Html {
    let year = use_state(|| None::<u16>);
    let season = use_state(|| None::<u8>);
    let day = use_state(|| None::<u8>);

    let year_updated = {
        let year = year.clone();
        let season = season.clone();
        let day = day.clone();
        let updated = properties.updated.clone();
        Callback::from(move |event: InputEvent| {
            let new_year = event
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u16>()
                .ok()
                .filter(|&y| y > 0);
            year.set(new_year);
            updated.emit(compute_date(new_year, *season, *day));
        })
    };

    let season_updated = {
        let year = year.clone();
        let season = season.clone();
        let day = day.clone();
        let updated = properties.updated.clone();
        Callback::from(move |event: Event| {
            let index = event
                .target_unchecked_into::<HtmlSelectElement>()
                .selected_index();
            let new_season = if (1..=4).contains(&index) {
                Some(index as u8)
            } else {
                None
            };
            season.set(new_season);
            updated.emit(compute_date(*year, new_season, *day));
        })
    };

    let day_updated = {
        let year = year.clone();
        let season = season.clone();
        let day = day.clone();
        let updated = properties.updated.clone();
        Callback::from(move |event: InputEvent| {
            let new_day = event
                .target_unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u8>()
                .ok()
                .filter(|&d| (1..=28).contains(&d));
            day.set(new_day);
            updated.emit(compute_date(*year, *season, new_day));
        })
    };

    html! {
        <div class="field">
            <label class="label">{ properties.label.clone() }</label>
            <div class="control">
                <div class="field has-addons">
                    <div class="control is-expanded">
                        <input class="input" oninput={ day_updated } placeholder="Day" type="text" value={ day.map(|d| d.to_string()).unwrap_or_default() } />
                    </div>
                    <div class="control">
                        <div class="select">
                            <select onchange={ season_updated }>
                                <option disabled=true hidden=true selected={ season.unwrap_or(0) == 0 }>{ "Season" }</option>
                                <option selected={ season.unwrap_or(0) == 1 }>{ "Spring" }</option>
                                <option selected={ season.unwrap_or(0) == 2 }>{ "Summer" }</option>
                                <option selected={ season.unwrap_or(0) == 3 }>{ "Fall" }</option>
                                <option selected={ season.unwrap_or(0) == 4 }>{ "Winter" }</option>
                            </select>
                        </div>
                    </div>
                    <div class="control is-expanded">
                        <input class="input" oninput={ year_updated } placeholder="Year" type="text" value={ year.map(|y| y.to_string()).unwrap_or_default() } />
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Date jump: shows a "Jump" button, emits `i32` only on click.
#[derive(Properties, PartialEq)]
pub struct DateJumpProperties {
    pub updated: Callback<i32>,
}

#[component]
pub fn DateJump(properties: &DateJumpProperties) -> Html {
    let year = use_state(|| None::<u16>);
    let season = use_state(|| None::<u8>);
    let day = use_state(|| None::<u8>);

    let year_updated = {
        let year = year.clone();
        Callback::from(move |event: InputEvent| {
            year.set(
                event
                    .target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .parse::<u16>()
                    .ok()
                    .filter(|&y| y > 0),
            );
        })
    };

    let season_updated = {
        let season = season.clone();
        Callback::from(move |event: Event| {
            let index = event
                .target_unchecked_into::<HtmlSelectElement>()
                .selected_index();
            season.set(if (1..=4).contains(&index) {
                Some(index as u8)
            } else {
                None
            });
        })
    };

    let day_updated = {
        let day = day.clone();
        Callback::from(move |event: InputEvent| {
            day.set(
                event
                    .target_unchecked_into::<HtmlInputElement>()
                    .value()
                    .parse::<u8>()
                    .ok()
                    .filter(|&d| (1..=28).contains(&d)),
            );
        })
    };

    let jump_updated = {
        let year = year.clone();
        let season = season.clone();
        let day = day.clone();
        let updated = properties.updated.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(date) = compute_date(*year, *season, *day) {
                updated.emit(date);
            }
        })
    };

    html! {
        <div class="field has-addons">
            <div class="control is-expanded">
                <input class="input" oninput={ day_updated } placeholder="Day" type="text" value={ day.map(|d| d.to_string()).unwrap_or_default() } />
            </div>
            <div class="control">
                <div class="select">
                    <select onchange={ season_updated }>
                        <option disabled=true hidden=true selected={ season.unwrap_or(0) == 0 }>{ "Season" }</option>
                        <option selected={ season.unwrap_or(0) == 1 }>{ "Spring" }</option>
                        <option selected={ season.unwrap_or(0) == 2 }>{ "Summer" }</option>
                        <option selected={ season.unwrap_or(0) == 3 }>{ "Fall" }</option>
                        <option selected={ season.unwrap_or(0) == 4 }>{ "Winter" }</option>
                    </select>
                </div>
            </div>
            <div class="control is-expanded">
                <input class="input" oninput={ year_updated } placeholder="Year" type="text" value={ year.map(|y| y.to_string()).unwrap_or_default() } />
            </div>
            <div class="control">
                <button class="button is-primary" onclick={ jump_updated }>{ "Jump" }</button>
            </div>
        </div>
    }
}
