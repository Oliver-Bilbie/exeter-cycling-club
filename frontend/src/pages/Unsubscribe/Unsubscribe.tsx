import React, { useEffect, useState } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { Box, Header, Button } from "grommet";

import LoadingSpinner from "../../components/LoadingSpinner/LoadingSpinner";
import { callApi } from "../../helpers/callApi";
import { EMAIL_ENDPOINT } from "../../constants/endpoints";

const Unsubscribe: React.FC = (): React.ReactElement => {
    const [loading, setLoading] = useState(true);
    const [success, setSuccess] = useState(false);

    // Extract request code from query parameters
    const [userParams] = useSearchParams();
    const emailId = userParams.get("id") as string;

    const handleResponse = (response): void => {
        setSuccess(response.body.status === 200);
        setLoading(false);
    };

    useEffect(() => {
        // Call our unsubscribe endpoint
        callApi(
            "DELETE",
            EMAIL_ENDPOINT,
            handleResponse,
            undefined,
            undefined,
            `{"id": "${emailId}"}`
        );
    }, []);

    return (
        <Box background="background" align="center" justify="center" fill>
            {loading ? (
                <LoadingSpinner size="200px" />
            ) : (
                <Box gap="medium" align="center" justify="center">
                    <Header>
                        {success
                            ? "Successfully unsubscribed from the mailing list"
                            : "Unable to unsubscribe"}
                    </Header>
                    <Link to={"/"}>
                        <Button label="Home" primary alignSelf="center" />
                    </Link>
                </Box>
            )}
        </Box>
    );
};

export default Unsubscribe;
