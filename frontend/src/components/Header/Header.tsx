import React from "react";
import { Image, Stack } from "grommet";

import Banner from "../../components/Banner/Banner";
import Showcase from "../../components/Showcase/Showcase";

interface HeaderProps {
    title: string;
    buttonText?: string;
    buttonLink?: string;
    width: number;
    compact?: boolean;
}

const Header: React.FC<HeaderProps> = ({
    title,
    buttonText,
    buttonLink,
    width,
    compact,
}): React.ReactElement => {
    return (
        <Stack fill interactiveChild="first">
            <>
                <Banner />
                <Showcase
                    mainText={title}
                    buttonText={buttonText}
                    buttonLink={buttonLink}
                    width={width}
                    compact={compact}
                />
            </>
            <Image
                margin={{ left: "medium", right: "medium" }}
                src={require("../../images/logo_transparent.png")}
                alignSelf="start"
                width={width > 600 ? 200 : width/3}
            />
        </Stack>
    );
};

export default Header;
