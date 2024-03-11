#[derive(PartialEq, Clone)]
pub struct AboutUsSection {
    pub title: &'static str,
    pub body: [&'static str; 5],
}

pub const ABOUT_US: AboutUsSection = AboutUsSection {
    title: "About us",
    body: [
        "Welcome to Exeter Cycling Club, bringing together people from Exeter and the surrounding area with a shared interest in group cycling. The best way to describe us is a 'cafe club'. We love cycling and we equally enjoy a social cafe stop, usually somewhere in the middle of the ride.",
        "The focus of the group is on developing the sociable element of cycling; having fun, getting to know people and supporting each other. We organise a weekly ride on Sunday of between 40-60 miles at an average pace of 16-18mph. Meeting up at Pinhoe Sainsbury's we start rides at 7.30am (summer start time) so that we are back just after 11.00am and we always include a coffee stop en route.",
        "There is a strict no-drop policy, rides are at the pace of the slowest rider and we wait at the top of climbs for each other. We like to keep rides rolling, aim to keep stops to a minimum and get back in good time so that members can balance cycling with other family commitments Please feel free to get in touch and come and join us, we are always eager to welcome new riders. We are not an official club, there is no committee, rides are arranged by the riders and there are no joining fees to pay.",
        "",
        "",
    ],
};

pub const JOIN_US_ON_A_RIDE: AboutUsSection = AboutUsSection {
    title: "Join us on a ride",
    body: [
        "Exeter Cycling Club organises regular Sunday group rides of around 50 miles at an approximate pace of 15 - 19 mph depending on the group and terrain. Group rides help build cycling endurance and to keep motivation with great camaraderie and support between riders.",
        "Starting from Pinhoe Sainsbury's is perfect for rides taking in East Devon, the Teign Valley, Exe Valley and Dartmoor. This central starting point gives good opportunities for rides all over Devon. We try and keep rides to about 50 miles as this is the best balance between time on the bike and building cycling endurance. If you can cycle 30-40 miles solo you can quickly gain the fitness for 50 mile rides in a group.",
        "Exeter Cycling Club was started by a group of cycle enthusiasts who enjoy riding as a group at a reasonable pace but don't take things too seriously. The main focus of the club is to develop rides that are enjoyable to all and allow riders to keep rolling with not too many stops other than re-grouping at the top of climbs. This allows our riders to get home at a reasonable time and to balance family life and other commitments around their cycling.",
        "",
        "",
    ],
};

pub const RIDING_GUIDELINES: AboutUsSection = AboutUsSection {
    title: "Riding guidelines",
    body: [
        "If you're not used to riding in a large group, rolling away handlebar to handlebar with other riders can sometimes be intimidating. However, with some knowledge of what to expect, the experience will be fun, sociable and safe for everybody in your group.",
        "All riders should follow the instructions and guidance offered by Ride Leaders on rides. Remember that the Ride Leader is considering the safety of everyone on a ride and following a tried and tested format that has been widely praised as being highly effective. Refusing to follow Ride Leader instructions or arguing with them during the ride is not acceptable.",
        "All riders should act with consideration and respect towards their fellow cyclists as well as other road users to help ensure the safety and enjoyment of the rides.",
        "For each ride, there will be at least one Ride Leader. The Ride Leader will be responsible for ensuring that the group stays together, that the speed is right for the group level and clear directions are given ahead of turns. The Ride Leader will ensure that the ride is ridden as close to the advertised speed as possible. It is each rider's responsibility to place themselves in a group that is appropriate for their CURRENT level of fitness and ability. Pushing the pace in a group that cannot go faster or slowing down a group because you cannot keep up are equally disruptive behaviours and can compromise group safety and the enjoyment of other riders.",
        "Safety is always our first priority and if you are unsure when you turn up to a club ride, seek the advice of the group coordinator, a Ride Leader or another rider. We are a very friendly bunch and most of us picked up these guidelines from riding and chatting with more experienced riders!",
    ],
};
