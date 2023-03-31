import React, { ReactElement, useEffect, useState } from "react";
import { Box, Heading, Text, Button, RangeInput } from "grommet";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { solid } from "@fortawesome/fontawesome-svg-core/import.macro";
import GpxParser from "gpxparser";
import { LatLngExpression } from "leaflet";

import Map from "../../components/Map/Map";
import ElevationView from "../ElevationView/ElevationView";
import LoadingSpinner from "../LoadingSpinner/LoadingSpinner";
import { RouteType } from "../../types";
import Theme from "../../theme";

interface RideDetailsProps {
    routeData: RouteType;
    width: number;
}

const RideDetails: React.FC<RideDetailsProps> = ({
    routeData,
    width,
}): React.ReactElement => {
    // Pixel width of the page below which the component will render as a column
    const WIDTH_THRESHOLD = 1050;

    const [loading, setLoading] = useState(true);
    const [route, setRoute] = useState({
        coordinates: [] as LatLngExpression[],
        elevation: [] as number[],
        center: [0, 0] as LatLngExpression,
    });
    const [highlight, setHighlight] = useState({
        index: 0,
        coordinates: [0, 0] as LatLngExpression,
    });

    const updateHighlight = (event): void => {
        const index = Math.floor(
            (event.target.value * route.coordinates.length) / 101
        );
        setHighlight({
            index: event.target.value,
            coordinates: route.coordinates[index],
        });
    };

    useEffect(() => {
        if (routeData.gpx !== "") {
            // Load and parse route data
            const gpx = new GpxParser();
            gpx.parse(routeData.gpx);

            const routeCoordinates = gpx.tracks[0].points.map(
                (p): LatLngExpression => [p.lat, p.lon]
            );
            const routeElevation = gpx.tracks[0].points.map(
                (p): number => p.ele
            );

            // Find center point
            const routeLat = routeCoordinates.map((p): number => p[0]);
            const routeLon = routeCoordinates.map((p): number => p[1]);
            /* eslint-disable prefer-spread */
            const maxLat = Math.max.apply(Math, routeLat);
            const minLat = Math.min.apply(Math, routeLat);
            const maxLon = Math.max.apply(Math, routeLon);
            const minLon = Math.min.apply(Math, routeLon);
            const routeCenter = [
                (maxLat + minLat) / 2,
                (maxLon + minLon) / 2,
            ] as LatLngExpression;

            setRoute({
                coordinates: routeCoordinates,
                elevation: routeElevation,
                center: routeCenter,
            });

            setLoading(false);
        }
    }, [routeData.gpx]);

    return (
        <Box
            direction={width > WIDTH_THRESHOLD ? "row" : "column-reverse"}
            pad="large"
            gap="large"
            justify="center"
        >
            <Box
                width={
                    width > WIDTH_THRESHOLD
                        ? `${(3 * width) / 7}px`
                        : `${width}px`
                }
                height={width > WIDTH_THRESHOLD ? undefined : `${width / 2}px`}
                elevation="medium"
                animation={["fadeIn", "slideUp"]}
            >
                {loading ? (
                    <Box fill justify="center" background="text-light">
                        <LoadingSpinner size="200px" />
                    </Box>
                ) : (
                    <Map
                        route={route.coordinates}
                        center={route.center}
                        zoom={
                            width > WIDTH_THRESHOLD ? 11 : width > 500 ? 10 : 9
                        }
                        highlight={highlight.coordinates}
                    />
                )}
            </Box>
            <Box
                width={
                    width > WIDTH_THRESHOLD
                        ? `${(3 * width) / 7}px`
                        : `${width}px`
                }
                direction="column"
                justify="center"
                gap="medium"
                animation={["fadeIn", "slideUp"]}
            >
                <Heading margin="none" responsive={false}>
                    {routeData.name}
                </Heading>
                <Box direction="row" gap="small">
                    <FontAwesomeIcon
                        icon={solid("road")}
                        color={Theme.global.colors["accent"]}
                        size="2x"
                    />
                    <Text size="large" weight="bold" alignSelf="center">
                        {routeData.distance}
                    </Text>
                    <Box width="15px" />
                    <FontAwesomeIcon
                        icon={solid("mountain")}
                        color={Theme.global.colors["accent"]}
                        size="2x"
                    />
                    <Text size="large" weight="bold" alignSelf="center">
                        {routeData.elevation_gain}
                    </Text>
                </Box>
                {routeData.description
                    .split("$NEWLINE")
                    .map((paragraph: string, index: number): ReactElement => {
                        return <Text key={index}>{paragraph}</Text>;
                    })}
                <Box width="small">
                    <Button
                        label="Download GPX"
                        primary
                        href="https://s3.eu-west-1.amazonaws.com/ecc.oliver-bilbie.co.uk/eccRoute.gpx"
                        alignSelf="start"
                    />
                </Box>
                <Box height="10px" />
                <ElevationView elevationProfile={route.elevation} />
                <RangeInput
                    value={highlight.index}
                    onChange={updateHighlight}
                />
            </Box>
        </Box>
    );
};

export default RideDetails;
