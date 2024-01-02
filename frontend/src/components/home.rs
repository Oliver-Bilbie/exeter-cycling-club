use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Route;
use crate::components::nav_bar::NavBar;
use crate::helpers::go_to_page::go_to_page;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <div>
            <section class="is-fullheight">
                <NavBar is_sticky={true} />

                <section class="fullheight-bg hero is-fullheight-with-navbar" style="overflow: hidden;">
                    <div class="is-overlay hero-body">
                        <div class="container has-text-centered">
                            <h1 class="title is-1 has-text-light">
                                    {"Exeter Cycling Club" }
                            </h1>
                            <h3 class="subtitle is-4 has-text-light">
                                { "The best cycling club in the world!" }
                            </h3>
                            <div class="block">
                                <button
                                    class="button is-primary m-2"
                                    onclick={
                                        go_to_page(navigator.clone(), Route::About)
                                    }
                                >
                                    { "About Us" }
                                </button>
                                <button
                                    class="button m-2"
                                    onclick={
                                        go_to_page(navigator.clone(), Route::RidePage)
                                    }
                                >
                                    { "Upcoming Ride" }
                                </button>
                            </div>
                        </div>
                    </div>
                </section>
            </section>

            // TODO: For the love of all that is holy - refactor the following three blocks thoroughly
            <section class="section texture-light">
                <div class="container columns is-desktop">
                    <div class="column">
                        <h1 class="title is-1 has-text-centered">
                            { "About us" }
                        </h1>
                        <p class="has-text-centered m-2">
                            {
                                "Welcome to Exeter Cycling Club, bringing together people from Exeter and the surrounding area with a shared interest in group cycling. The best way to describe us is a 'cafe club'. We love cycling and we equally enjoy a social cafe stop, usually somewhere in the middle of the ride."
                            }
                        </p>
                        <p class="has-text-centered m-2">
                            {
                                "The focus of the group is on developing the sociable element of cycling; having fun, getting to know people and supporting each other. We organise a weekly ride on Sunday of between 40-60 miles at an average pace of 16-18mph. Meeting up at Pinhoe Sainsbury's we start rides at 7.30am (summer start time) so that we are back just after 11.00am and we always include a coffee stop en route."
                            }
                        </p>
                        <p class="has-text-centered m-2">
                            {
                                "There is a strict no-drop policy, rides are at the pace of the slowest rider and we wait at the top of climbs for each other. We like to keep rides rolling, aim to keep stops to a minimum and get back in good time so that members can balance cycling with other family commitments Please feel free to get in touch and come and join us, we are always eager to welcome new riders. We are not an official club, there is no committee, rides are arranged by the riders and there are no joining fees to pay."
                            }
                        </p>
                    </div>
                    <div class="column" style="display: flex;">
                        // TODO: Refactor into framed image component
                        <div class="box p-2 has-background-dark" style="display: flex;">
                            <img src="images/home1.jpg" style="object-fit: cover;" />
                        </div>
                    </div>
                </div>
            </section>

            <section class="section texture-dark">
                <div class="container columns is-desktop">
                    <div class="column is-hidden-mobile is-hidden-tablet-only" style="display: flex;">
                        <div class="box p-2 has-background-dark" style="display: flex;">
                            <img src="images/home2.jpg" style="object-fit: cover;" />
                        </div>
                    </div>
                    <div class="column">
                        <h1 class="title is-1 has-text-centered has-text-light">
                            { "Join us on a ride" }
                        </h1>
                        <p class="has-text-centered has-text-light m-2">
                            {
                                "Exeter Cycling Club organises regular Sunday group rides of around 50 miles at an approximate pace of 15 - 19 mph depending on the group and terrain. Group rides help build cycling endurance and to keep motivation with great camaraderie and support between riders."
                            }
                        </p>
                        <p class="has-text-centered has-text-light m-2">
                            {
                                "Starting from Pinhoe Sainsbury's is perfect for rides taking in East Devon, the Teign Valley, Exe Valley and Dartmoor. This central starting point gives good opportunities for rides all over Devon. We try and keep rides to about 50 miles as this is the best balance between time on the bike and building cycling endurance. If you can cycle 30-40 miles solo you can quickly gain the fitness for 50 mile rides in a group."
                            }
                        </p>
                        <p class="has-text-centered has-text-light m-2">
                            {
                                "Exeter Cycling Club was started by a group of cycle enthusiasts who enjoy riding as a group at a reasonable pace but don't take things too seriously. The main focus of the club is to develop rides that are enjoyable to all and allow riders to keep rolling with not too many stops other than re-grouping at the top of climbs. This allows our riders to get home at a reasonable time and to balance family life and other commitments around their cycling."
                            }
                        </p>
                    </div>
                    <div class="column is-hidden-desktop" style="display: flex;">
                        <div class="box p-2 has-background-dark" style="display: flex;">
                            <img src="images/home2.jpg" style="object-fit: cover;" />
                        </div>
                    </div>
                </div>
            </section>

            <section class="section texture-light">
                <div class="container columns is-desktop">
                    <div class="column">
                        <h1 class="title is-1 has-text-centered">
                            { "Riding guidelines" }
                        </h1>
                        <p class="has-text-centered m-2">
                            {
                                "If you're not used to riding in a large group, rolling away handlebar to handlebar with other riders can sometimes be intimidating. However, with some knowledge of what to expect, the experience will be fun, sociable and safe for everybody in your group."
                            }
                        </p>
                        <p class="has-text-centered m-2">
                            {
                                "All riders should follow the instructions and guidance offered by Ride Leaders on rides. Remember that the Ride Leader is considering the safety of everyone on a ride and following a tried and tested format that has been widely praised as being highly effective. Refusing to follow Ride Leader instructions or arguing with them during the ride is not acceptable."
                            }
                        </p>
                        <p class="has-text-centered m-2">
                            {
                                "All riders should act with consideration and respect towards their fellow cyclists as well as other road users to help ensure the safety and enjoyment of the rides."
                            }
                        </p>
                        <p class="has-text-centered m-2">
                            {
                                "For each ride, there will be at least one Ride Leader. The Ride Leader will be responsible for ensuring that the group stays together, that the speed is right for the group level and clear directions are given ahead of turns. The Ride Leader will ensure that the ride is ridden as close to the advertised speed as possible. It is each rider's responsibility to place themselves in a group that is appropriate for their CURRENT level of fitness and ability. Pushing the pace in a group that cannot go faster or slowing down a group because you cannot keep up are equally disruptive behaviours and can compromise group safety and the enjoyment of other riders."
                            }
                        </p>
                        <p class="has-text-centered m-2">
                            {
                                "Safety is always our first priority and if you are unsure when you turn up to a club ride, seek the advice of the group coordinator, a Ride Leader or another rider. We are a very friendly bunch and most of us picked up these guidelines from riding and chatting with more experienced riders!"
                            }
                        </p>
                    </div>
                    <div class="column" style="display: flex;">
                        <div class="box p-2 has-background-dark" style="display: flex;">
                            <img src="images/home3.jpg" style="object-fit: cover;" />
                        </div>
                    </div>
                </div>
            </section>

            // TODO: Refactor into footer component
            // TODO: Add social media links
            // TODO: Add sitemap
            <footer class="footer texture-dark">
                <div class="content has-text-centered">
                    <p class="has-text-light">
                        { "Exeter Cycling Club © 2024" }
                    </p>
                </div>
            </footer>
        </div>
    }
}
