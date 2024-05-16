use yew::prelude::*;

use crate::components::configuration_form::ConfigurationForm;
use crate::components::tabs::Tabs;
use crate::configuration::Configuration;
use crate::implementations::geodes::Geodes;
use crate::implementations::joja::Joja;
use crate::implementations::krobus::Krobus;
use crate::implementations::pierre::Pierre;
use crate::implementations::sandy::Sandy;
use crate::implementations::traveling_cart::TravelingCart;
use crate::implementations::util::Implementation;

#[function_component]
pub fn App() -> Html {
    let configuration: UseStateHandle<Option<Configuration>> = use_state_eq(|| None);
    let implementation: UseStateHandle<Option<Implementation>> =
        use_state_eq(|| None);

    let configuration_updated: Callback<Configuration> = use_callback(
        (configuration.clone(), implementation.clone()),
        move |configuration_: Configuration,
              (configuration, implementation): &(UseStateHandle<Option<Configuration>>, UseStateHandle<Option<Implementation>>)| {
            configuration.set(Some(configuration_));
            implementation.set(None);  // Resetting the tabs when "Go" is clicked means that all implementatiosn will be created a.new.
        },
    );


    let implementation_updated: Callback<Implementation> = use_callback(
        implementation.clone(),
        move |implementation_: Implementation, implementation: &UseStateHandle<Option<Implementation>>| {
            implementation.set(Some(implementation_));
        },
    );

    html!(
        <>
            <ConfigurationForm updated={ configuration_updated } />
            {
                match (*configuration).clone() {
                    Some(configuration) => html!(
                        <section class="section">
                            <h1 class="title">{ "Results" }</h1>
                            <div class="container">
                                <Tabs<Implementation> tabs={ vec![Implementation::TravelingCart, Implementation::Krobus, Implementation::Sandy, Implementation::Pierre, Implementation::Joja, Implementation::Geodes] } selected={ *implementation } updated={ implementation_updated } />
                                {
                                    match *implementation {
                                        None => html!(),
                                        Some(Implementation::TravelingCart) => html!(
                                            <TravelingCart configuration={ configuration.clone() } />
                                        ),
                                        Some(Implementation::Krobus) => html!(
                                            <Krobus configuration={ configuration.clone() } />
                                        ),
                                        Some(Implementation::Sandy) => html!(
                                            <Sandy configuration={ configuration.clone() } />
                                        ),
                                        Some(Implementation::Pierre) => html!(
                                            <Pierre configuration={ configuration.clone() } />
                                        ),
                                        Some(Implementation::Joja) => html!(
                                            <Joja configuration={ configuration.clone() } />
                                        ),
                                        Some(Implementation::Geodes) => html!(
                                            <Geodes configuration={ configuration.clone() } />
                                        ),
                                    }
                                }
                            </div>
                        </section>
                    ),
                    None => html!(),
                }
            }
        </>
    )
}
