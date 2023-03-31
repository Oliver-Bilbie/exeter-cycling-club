import React, { useState } from "react";
import { Link } from "react-router-dom";
import { useSelector } from "react-redux";
import { Box, Heading, Text, Button, TextArea } from "grommet";
import { Close, Run } from "grommet-icons";

import LoadingSpinner from "../../components/LoadingSpinner/LoadingSpinner";
import { callApi } from "../../helpers/callApi";
import { ROUTE_ENDPOINT } from "../../constants/endpoints";

enum STATUS {
    "IDLE",
    "LOADING",
    "SUCCESS",
    "FAILED",
}

const Cancel: React.FC = (): React.ReactElement => {
    /* eslint-disable  @typescript-eslint/no-explicit-any */
    const user = useSelector((state: any) => {
        return state.user;
    });

    const [message, setMessage] = useState("");
    const [formStatus, setFormStatus] = useState(STATUS.IDLE);

    const handleResponse = (response): void => {
        setFormStatus(
            response.body.status === 200 ? STATUS.SUCCESS : STATUS.FAILED
        );
    };

    const handleSubmit = (): void => {
        // Call our cancel endpoint
        setFormStatus(STATUS.LOADING);
        callApi(
            "DELETE",
            ROUTE_ENDPOINT,
            handleResponse,
            undefined,
            undefined,
            `{"access_token": "${
                user.access_token
            }", "message": "${message.replace(/\n/g, "$NEWLINE")}"}`
        );
    };

    return (
        <Box background="background" align="center" justify="center" fill>
            {formStatus === STATUS.IDLE && (
                <Box gap="medium" align="center" justify="center">
                    <Text weight="bold">
                        Please provide a message regarding the cancellation
                    </Text>
                    <Box height="200px" width="400px">
                        <TextArea
                            name="message"
                            value={message}
                            onChange={(event): void =>
                                setMessage(event.target.value)
                            }
                            plain={false}
                            resize={false}
                            fill
                        />
                    </Box>
                    <Box direction="row" gap="medium">
                        <Link to={"/"}>
                            <Button
                                label="Home"
                                icon={<Run />}
                                primary
                                reverse
                                alignSelf="center"
                            />
                        </Link>
                        <Button
                            label="Cancel Route"
                            icon={<Close />}
                            onClick={handleSubmit}
                            primary
                            reverse
                            alignSelf="center"
                        />
                    </Box>
                </Box>
            )}
            {formStatus === STATUS.LOADING && <LoadingSpinner size="200px" />}
            {formStatus === STATUS.SUCCESS && (
                <Box gap="medium" align="center" justify="center">
                    <Heading>Route was successfully cancelled</Heading>
                    <Link to={"/"}>
                        <Button label="Home" primary alignSelf="center" />
                    </Link>
                </Box>
            )}
            {formStatus === STATUS.FAILED && (
                <Box gap="medium" align="center" justify="center">
                    <Heading>Unable to cancel route</Heading>
                    <Link to={"/"}>
                        <Button label="Home" primary alignSelf="center" />
                    </Link>
                </Box>
            )}
        </Box>
    );
};

export default Cancel;
