import { Image, Spinner } from "grommet";
import React from "react";

interface LoadingSpinnerProps {
    size: string;
}

const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
    size,
}): React.ReactElement => {
    return (
        <Spinner
            animation={{
                type: "rotateRight",
                duration: 1500,
                size: "xsmall",
            }}
            size={size}
            justify="center"
            alignSelf="center"
        >
            <Image src={require("../../images/logo_transparent.png")} />
        </Spinner>
    );
};

export default LoadingSpinner;
