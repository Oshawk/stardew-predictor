use yew::prelude::*;

use crate::components::configuration_form::ConfigurationForm;
use crate::components::tabs::Tabs;
use crate::configuration::Configuration;
use crate::implementations::krobus::Krobus;
use crate::implementations::sandy::Sandy;
use crate::implementations::traveling_cart::TravelingCart;
use crate::implementations::util::Implementation;

#[function_component]
pub fn App() -> Html {
    let configuration: UseStateHandle<Option<Configuration>> = use_state_eq(|| None);
    let configuration_updated: Callback<Configuration> = use_callback(
        configuration.clone(),
        move |configuration_: Configuration,
              configuration: &UseStateHandle<Option<Configuration>>| {
            configuration.set(Some(configuration_));
        },
    );

    let implementation: UseStateHandle<Implementation> =
        use_state_eq(|| Implementation::TravelingCart);
    let implementation_updated: Callback<Implementation> = use_callback(
        implementation.clone(),
        move |implementation_: Implementation, implementation: &UseStateHandle<Implementation>| {
            implementation.set(implementation_);
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
                                <Tabs<Implementation> tabs={ vec![Implementation::TravelingCart, Implementation::Krobus, Implementation::Sandy] } updated={ implementation_updated } />
                                {
                                    match *implementation {
                                        Implementation::TravelingCart => html!(
                                            <TravelingCart configuration={ configuration.clone() } />
                                        ),
                                        Implementation::Krobus => html!(
                                            <Krobus configuration={ configuration.clone() } />
                                        ),
                                        Implementation::Sandy => html!(
                                            <Sandy configuration={ configuration.clone() } />
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
