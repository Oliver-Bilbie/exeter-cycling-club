import React, { useState } from "react";
import { Box, Button, Text, TextInput } from "grommet";

import LoadingSpinner from "../LoadingSpinner/LoadingSpinner";
import { callApi } from "../../helpers/callApi";
import { EMAIL_ENDPOINT } from "../../constants/endpoints";

const EmailSignUp: React.FC = (): React.ReactElement => {
    const [name, setName] = useState("");
    const [email, setEmail] = useState({
        value: "",
        valid: false,
        error: false,
    });
    const [loading, setLoading] = useState(false);
    const [success, setSuccess] = useState(false);

    const handleUpdateEmail = (event): void => {
        const newEmail = event.target.value;
        const isValid = newEmail
            .toLowerCase()
            .match(
                /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
            );
        setEmail({ ...email, value: newEmail, valid: isValid });
    };

    const handleSubscriptionOutcome = (response): void => {
        if (response.body.status === 200) {
            setSuccess(true);
        } else {
            setEmail({ ...email, error: true });
        }
        setLoading(false);
    };

    const handleSubscribe = (submitted_email: string): void => {
        setLoading(true);
        callApi(
            "PUT",
            EMAIL_ENDPOINT,
            handleSubscriptionOutcome,
            undefined,
            undefined,
            `{"email": "${submitted_email}", "name": "${name}"}`
        );
    };

    return (
        <Box
            background="background-dark"
            pad="medium"
            align="center"
            alignSelf="center"
            justify="center"
            width="420px"
            height="275px"
            round="medium"
        >
            {loading ? (
                <LoadingSpinner size="100px" />
            ) : (
                <>
                    {success ? (
                        <Text textAlign="center">
                            A confirmation email has been sent to {email.value}
                        </Text>
                    ) : (
                        <Box gap="small" align="center">
                            <Text weight="bold">
                                {email.error
                                    ? "Subscription unsuccessful"
                                    : "Subscribe to email alerts"}
                            </Text>
                            <Box gap="small" margin="small">
                                <TextInput
                                    value={name}
                                    onChange={(event): void =>
                                        setName(event.target.value)
                                    }
                                    placeholder="Name"
                                    textAlign="center"
                                    width="350px"
                                />
                                <TextInput
                                    value={email.value}
                                    onChange={handleUpdateEmail}
                                    placeholder="Email address"
                                    textAlign="center"
                                    width="350px"
                                />
                            </Box>
                            <Button
                                label="Subscribe"
                                onClick={(): void =>
                                    handleSubscribe(email.value)
                                }
                                primary
                                color="brand"
                                disabled={!email.valid || name === ""}
                            />
                        </Box>
                    )}
                </>
            )}
        </Box>
    );
};

export default EmailSignUp;
