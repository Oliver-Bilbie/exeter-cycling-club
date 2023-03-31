import React from "react";
import { DataChart } from "grommet";

interface ElevationViewProps {
    elevationProfile: number[];
}

const ElevationView: React.FC<ElevationViewProps> = ({
    elevationProfile,
}): React.ReactElement => {
    return (
        <DataChart
            data={elevationProfile.map(
                (point: number): { elevation: number } => {
                    return { elevation: point };
                }
            )}
            series="elevation"
            chart={{
                property: "elevation",
                type: "area",
                color: "background-dark",
            }}
            axis={false}
            size={{ width: "fill", height: "small" }}
            gap="none"
        />
    );
};

export default ElevationView;
