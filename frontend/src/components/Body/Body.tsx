import React from "react";
import BodySection from "../BodySection/BodySection";

import home1 from "../../images/home1.jpg";
import home2 from "../../images/home2.jpg";
import home3 from "../../images/home3.jpg";
import { ABOUT_US, JOIN_US, GUIDELINES } from "../../copy/homepage";

const CONTENT = [
    {
        title: "About us",
        text: ABOUT_US,
        image: home1,
    },
    {
        title: "Join us on a ride",
        text: JOIN_US,
        image: home2,
    },
    {
        title: "Riding guidelines",
        text: GUIDELINES,
        image: home3,
    },
];

interface BodyProps {
    width: number;
}

const Body: React.FC<BodyProps> = ({ width }): React.ReactElement => {
    // Pixel width of the page below which the component will render as a column
    const WIDTH_THRESHOLD = 1050;

    return (
        <>
            {CONTENT.map((entry, index) => {
                return (
                    <BodySection
                        title={entry.title}
                        text={entry.text}
                        image={entry.image}
                        color={
                            index % 2 === 0 ? "background" : "background-dark"
                        }
                        reverse={index % 2 === 0}
                        slim={width <= WIDTH_THRESHOLD}
                        key={index}
                    />
                );
            })}
        </>
    );
};

export default Body;
