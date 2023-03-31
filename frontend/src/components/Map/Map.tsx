import React from "react";
import {
    Circle,
    MapContainer,
    Marker,
    Polyline,
    Popup,
    TileLayer,
} from "react-leaflet";
import { LatLngExpression } from "leaflet";
import { Box, Text } from "grommet";

import "./Map.css";
import {
    START_LOCATION_NAME,
    START_LOCATION_POSTCODE,
    START_LOCATION_COORDS,
} from "../../constants/startDetails";

interface MapProps {
    route: LatLngExpression[];
    center: LatLngExpression;
    zoom: number;
    highlight: LatLngExpression;
}

const Map: React.FC<MapProps> = ({
    route,
    center,
    zoom,
    highlight,
}): React.ReactElement => {
    return (
        <Box
            border={{ color: "accent", size: "large" }}
            elevation="large"
            height={{ max: "800px" }}
        >
            <MapContainer
                className="main"
                center={center}
                zoom={zoom}
                scrollWheelZoom={true}
            >
                <TileLayer
                    attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                    url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />
                <Marker position={route[0]}>
                    <Popup>
                        <Text weight="bold">Start</Text>
                        {Math.abs(route[0][0] - START_LOCATION_COORDS[0]) <
                            0.01 &&
                            Math.abs(route[0][1] - START_LOCATION_COORDS[1]) <
                                0.01 && (
                                <>
                                    <br />
                                    <Text>{START_LOCATION_NAME}</Text>
                                    <br />
                                    <Text>{START_LOCATION_POSTCODE}</Text>
                                </>
                            )}
                    </Popup>
                </Marker>

                <Polyline
                    pathOptions={{ color: "#d3000a", weight: 5 }}
                    positions={[route]}
                />
                <Circle center={highlight} radius={200} />
            </MapContainer>
        </Box>
    );
};

export default Map;
