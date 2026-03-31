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

#[component]
pub fn App() -> Html {
    let configuration = use_state_eq(|| None::<Configuration>);
    let implementation = use_state_eq(|| None::<Implementation>);

    let configuration_updated = {
        let configuration = configuration.clone();
        let implementation = implementation.clone();
        Callback::from(move |config: Configuration| {
            configuration.set(Some(config));
            implementation.set(None); // Reset tabs so all implementations are created fresh.
        })
    };

    let implementation_updated = {
        let implementation = implementation.clone();
        Callback::from(move |impl_: Implementation| {
            implementation.set(Some(impl_));
        })
    };

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
