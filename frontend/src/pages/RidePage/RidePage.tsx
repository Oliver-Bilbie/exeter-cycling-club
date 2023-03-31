import React, { useState, useEffect, ReactElement } from "react";
import { Link } from "react-router-dom";
import { useSelector } from "react-redux";
import { Box, Button, Heading, Text } from "grommet";

import Header from "../../components/Header/Header";
import Footer from "../../components/Footer/Footer";
import RideDetails from "../../components/RideDetails/RideDetails";
import EmailSignUp from "../../components/EmailSignUp/EmailSignUp";
import { callApi } from "../../helpers/callApi";
import useWindowWidth from "../../helpers/useWindowWidth";
import { RouteType } from "../../types";
import { Update } from "grommet-icons";

const RidePage: React.FC = (): React.ReactElement => {
    /* eslint-disable  @typescript-eslint/no-explicit-any */
    const user = useSelector((state: any) => {
        return state.user;
    });

    const width = useWindowWidth();

    const [routeData, setRouteData] = useState({
        id: "",
        name: "",
        distance: "",
        elevation_gain: "",
        description: "",
        gpx: "",
    } as RouteType);
    const [status, setStatus] = useState("ready");
    const [message, setMessage] = useState("");
    const [reload, setReload] = useState(false);

    const WIDTH_THRESHOLD = 1050;

    const setRoute = (response): void => {
        if (response.status === 200) {
            setStatus(response.body.status);
            if (response.body.status === "ready") {
                setRouteData({
                    id: response.body.id,
                    name: response.body.name,
                    distance: response.body.distance,
                    elevation_gain: response.body.elevation_gain,
                    description: response.body.description,
                    gpx: response.body.gpx,
                });
            } else {
                setMessage(response.body.message);
            }
        } else {
            setStatus("missing");
        }
    };

    useEffect((): void => {
        // Load and parse route data on page load
        callApi(
            "GET",
            "https://s3.eu-west-1.amazonaws.com/ecc.oliver-bilbie.co.uk/routeData.json",
            setRoute,
            undefined,
            true
        );
    }, [reload]);

    return (
        <Box
            background="background"
            width={{ min: "400px" }}
            overflow={{ horizontal: "hidden" }}
            fill
        >
            <Header
                title="Upcoming Ride"
                buttonText="Contact Us"
                buttonLink="/contact"
                width={width}
            />
            <Box
                justify="start"
                height={
                    status === "ready"
                        ? {
                              min:
                                  width > WIDTH_THRESHOLD
                                      ? "1150px"
                                      : `${
                                            1700 - (WIDTH_THRESHOLD - width) / 2
                                        }px`,
                          }
                        : { min: "700px" }
                }
            >
                {status === "missing" && (
                    <Box align="center" margin="xlarge" gap="medium">
                        <Heading margin="none">Unable to load route</Heading>
                        <Text>
                            Please click retry or visit our Facebook page
                        </Text>
                        <Button
                            label="Retry"
                            onClick={(): void => setReload(!reload)}
                            icon={<Update />}
                            primary
                            reverse
                        />
                    </Box>
                )}
                {status === "ready" && (
                    <RideDetails routeData={routeData} width={width} />
                )}
                {(status === "cancelled" || status === "unavailable") && (
                    <Box align="center" margin="large">
                        <Heading margin="none">
                            {status === "cancelled" &&
                                "This week's ride has been cancelled"}
                            {status === "unavailable" &&
                                "This week's ride has not yet been announced"}
                        </Heading>
                        <Box height="15px" />
                        {message
                            .split("$NEWLINE")
                            .map(
                                (
                                    paragraph: string,
                                    index: number
                                ): ReactElement => {
                                    return <Text key={index}>{paragraph}</Text>;
                                }
                            )}
                    </Box>
                )}
                <EmailSignUp />
                {user.admin && (
                    <Box
                        height="100px"
                        align="center"
                        alignSelf="center"
                        alignContent="center"
                        direction="row"
                        gap="medium"
                    >
                        <Link to={"/select"}>
                            <Button
                                label="Set new route"
                                alignSelf="center"
                                primary
                            />
                        </Link>
                        {status === "ready" && (
                            <Link to={"/cancel"}>
                                <Button
                                    label="Cancel route"
                                    alignSelf="center"
                                    primary
                                />
                            </Link>
                        )}
                    </Box>
                )}
            </Box>
            <Box height={{ min: "20px" }} fill />
            <Footer width={width} />
        </Box>
    );
};

export default RidePage;
