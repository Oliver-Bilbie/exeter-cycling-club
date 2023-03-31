import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { useSelector } from "react-redux";
import { Box, Button, Heading, Text, Image, Layer, TextArea } from "grommet";
import { Bike, Close, Next, Previous, Run } from "grommet-icons";

import Header from "../../components/Header/Header";
import RouteSelect from "../../components/RouteSelect/RouteSelect";
import LoadingSpinner from "../../components/LoadingSpinner/LoadingSpinner";
import useWindowWidth from "../../helpers/useWindowWidth";
import { callApi } from "../../helpers/callApi";
import { ROUTE_ENDPOINT, STRAVA_API_ENDPOINT } from "../../constants/endpoints";
import { EMPTY_ROUTE, RouteType } from "../../types";
import { DEFAULT_DESCRIPTION } from "../../constants/defaultDescription";

const Select: React.FC = (): React.ReactElement => {
    /* eslint-disable  @typescript-eslint/no-explicit-any */
    const user = useSelector((state: any) => {
        return state.user;
    });

    const width = useWindowWidth();

    const [routes, setRoutes] = useState([] as RouteType[]);
    const [page, setPage] = useState(1);
    const [finalPage, setFinalPage] = useState({ found: false, number: 1 });
    const [selectedRoute, setSelectedRoute] = useState(EMPTY_ROUTE);
    const [showConfirm, setShowConfirm] = useState(false);
    const [loading, setLoading] = useState(true);
    const [message, setMessage] = useState({ show: false, body: "" });

    const handleRouteUpdate = (response): void => {
        if (response.body.status === 200) {
            setMessage({ show: true, body: response.body.body });
            setLoading(false);
        } else {
            setMessage({ show: true, body: response.body.body });
        }
    };

    const handleUpdateRoute = (): void => {
        if (user.id === "") {
            setMessage({ show: true, body: "Please sign in" });
        } else {
            setLoading(true);
            callApi(
                "PUT",
                ROUTE_ENDPOINT,
                handleRouteUpdate,
                undefined,
                undefined,
                `{ "access_token": "${user.access_token}", "id": "${
                    selectedRoute.id
                }", "description": "${selectedRoute.description.replace(
                    /\n/g,
                    "$NEWLINE"
                )}" }`
            );
        }
        setShowConfirm(false);
    };

    const handleSelectRoute = (route: RouteType): void => {
        if (user.id === "") {
            setMessage({ show: true, body: "Please sign in" });
        } else {
            setSelectedRoute({ ...route, description: DEFAULT_DESCRIPTION });
            setShowConfirm(true);
        }
    };

    const handleNewPage = (direction: "forwards" | "backwards"): void => {
        const nextPage = direction === "forwards" ? page + 1 : page - 1;
        if (nextPage > 0) {
            setPage(nextPage);
        }
    };

    const handleRouteData = (response): void => {
        if (response.body.length > 0) {
            const cleanRoutes = response.body.map((route: any) => {
                return {
                    id: route.id_str,
                    name: route.name,
                    updated: route.updated_at,
                    image: route.map_urls.url,
                };
            });
            setRoutes(cleanRoutes);
        } else {
            setPage(page - 1);
            setFinalPage({ found: true, number: page - 1 });
        }
        setLoading(false);
    };

    useEffect(() => {
        if (user.id === "") {
            setMessage({ show: true, body: "Please sign in" });
        } else {
            // Get a list of routes from Strava
            setLoading(true);
            callApi(
                "GET",
                `${STRAVA_API_ENDPOINT}/athletes/${user.id}/routes?page=${page}&per_page=50`,
                handleRouteData,
                user.access_token
            );
        }
    }, [page]);

    return (
        <Box
            background="background"
            width={{ min: "400px" }}
            overflow={{ horizontal: "hidden" }}
        >
            <Header
                title=""
                buttonText={undefined}
                buttonLink={undefined}
                width={width}
                compact
            />
            {message.show ? (
                <Box align="center" margin="large">
                    <Heading alignSelf="center">{message.body}</Heading>
                    <Link to={"/"}>
                        <Button label="Home" primary />
                    </Link>
                </Box>
            ) : loading ? (
                <LoadingSpinner size="200px" />
            ) : (
                <>
                    <RouteSelect
                        width={width}
                        routes={routes}
                        onSelect={handleSelectRoute}
                    />
                    <Box direction="row" alignSelf="center" gap="medium">
                        <Link to={"/upcoming"}>
                            <Button label="Exit" icon={<Run />} primary reverse />
                        </Link>
                        {page > 1 && (
                            <Button
                                label="Previous"
                                icon={<Previous />}
                                onClick={(): void => handleNewPage("backwards")}
                                primary
                            />
                        )}
                        {(!finalPage.found || finalPage.number > page) && (
                            <Button
                                label="Next"
                                icon={<Next />}
                                onClick={(): void => handleNewPage("forwards")}
                                primary
                                reverse
                            />
                        )}
                    </Box>
                    <Box height="20px" />
                    <Image
                        src={require("../../images/api_logo_pwrdBy_strava_horiz_gray.png")}
                        alignSelf="center"
                    />
                </>
            )}
            {showConfirm && (
                <Layer onClickOutside={(): void => setShowConfirm(false)}>
                    <Box pad="medium" align="center" gap="medium">
                        <Heading margin="none">Confirm selection</Heading>
                        <Text size="large" weight="bold">
                            Route: {selectedRoute.name}
                        </Text>
                        <Text>Please provide a description below</Text>
                        <Box width="large" height="medium">
                            <TextArea
                                value={selectedRoute.description}
                                onChange={(event): void =>
                                    setSelectedRoute({
                                        ...selectedRoute,
                                        description: event.target.value,
                                    })
                                }
                                resize={false}
                                fill
                            />
                        </Box>
                        <Box direction="row" gap="medium">
                            <Button
                                label="Cancel"
                                icon={<Close />}
                                onClick={():void => setShowConfirm(false)}
                                primary
                                reverse
                            />
                            <Button
                                label="Set Route"
                                icon={<Bike />}
                                onClick={handleUpdateRoute}
                                primary
                                reverse
                            />
                        </Box>
                    </Box>
                </Layer>
            )}
        </Box>
    );
};

export default Select;
