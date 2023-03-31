import React from "react";
import { Link } from "react-router-dom";
import { Box, Anchor, Nav, Text, Button } from "grommet";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { brands } from "@fortawesome/fontawesome-svg-core/import.macro";
import Theme from "../../theme";

interface FooterProps {
    width: number;
}

const navBarItems = [
    {
        key: "strava",
        icon: (
            <FontAwesomeIcon
                icon={brands("strava")}
                color={Theme.global.colors["text-light"]}
                size="2x"
            />
        ),
        url: "https://www.strava.com/clubs/ECCSW",
    },
    {
        key: "facebook",
        icon: (
            <FontAwesomeIcon
                icon={brands("facebook")}
                color={Theme.global.colors["text-light"]}
                size="2x"
            />
        ),
        url: "https://www.facebook.com/Exeter-and-East-Devon-Cycling-Group-2263893193890118/",
    },
];

const siteMap = [
    { label: "Home", link: "/" },
    { label: "Upcoming Ride", link: "/upcoming" },
    { label: "Contact Us", link: "/contact" },
    { label: "Sign In", link: "/signin" },
];

// Pixel width of the page below which the component will be center-aligned
const WIDTH_THRESHOLD = 1050;

const Footer: React.FC<FooterProps> = ({ width }): React.ReactElement => {
    return (
        <Box
            align={width <= WIDTH_THRESHOLD ? "center" : "end"}
            flex={false}
            gap="medium"
            justify="between"
            background="accent"
            pad={{ left: "medium", right: "medium" }}
            border={{ side: "bottom", color: "accent", size: "xsmall" }}
        >
            <Box direction="column" align={width <= WIDTH_THRESHOLD ? "center" : "end"}>
                <Box pad={{ top: "medium", bottom: "none" }}>
                    <Text
                        textAlign={width <= WIDTH_THRESHOLD ? "center" : "end"}
                        color="text-light"
                        weight="bold"
                        margin={{ bottom: "xsmall" }}
                    >
                        Sitemap
                    </Text>
                    {siteMap.map((item) => (
                        <Link
                            to={item.link}
                            style={{ textDecoration: "none" }}
                            key={item.label}
                        >
                            <Box>
                                <Text
                                    textAlign={
                                        width <= WIDTH_THRESHOLD
                                            ? "center"
                                            : "end"
                                    }
                                    color="text-light"
                                >
                                    {item.label}
                                </Text>
                            </Box>
                        </Link>
                    ))}
                </Box>
                <Nav
                    direction="row"
                    pad={{ top: "medium", bottom: "none" }}
                    color="text-strong"
                >
                    {navBarItems.map((item) => (
                        <Anchor
                            key={item.key}
                            icon={item.icon}
                            onClick={(): void => {
                                window.open(item.url, "_blank");
                            }}
                            a11yTitle={item.key}
                        />
                    ))}
                </Nav>
                <Box pad={{ top: "medium", bottom: "medium" }}>
                    <Button href="https://github.com/Oliver-Bilbie">
                        <Box>
                            <Text textAlign="center" color="text-light">
                                Web design by Oliver Bilbie
                            </Text>
                        </Box>
                    </Button>
                </Box>
            </Box>
        </Box>
    );
};

export default Footer;
