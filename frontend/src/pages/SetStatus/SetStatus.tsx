import React, { useEffect, useState } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { Box, Header, Button } from "grommet";

import LoadingSpinner from "../../components/LoadingSpinner/LoadingSpinner";
import { callApi } from "../../helpers/callApi";
import { STATUS_ENDPOINT } from "../../constants/endpoints";

const SetStatus: React.FC = (): React.ReactElement => {
  const [loading, setLoading] = useState(true);
  const [success, setSuccess] = useState(false);

  // Extract request code from query parameters
  const [userParams] = useSearchParams();
  const emailId = userParams.get("id") as string;
  const status = userParams.get("status") as string;

  const handleResponse = (response): void => {
    setSuccess(response.body.status === 200);
    setLoading(false);
  };

  useEffect(() => {
    // Call our set status endpoint
    callApi(
      "PUT",
      STATUS_ENDPOINT,
      handleResponse,
      undefined,
      undefined,
      `{"id": "${emailId}", "status": "${status}"}`
    );
  }, []);

  return (
    <Box background="background" align="center" justify="center" fill>
      {loading ? (
        <LoadingSpinner size="200px" />
      ) : (
        <Box gap="medium" align="center" justify="center">
          <Header>
            {success ? "Successfully set status" : "Unable to set status"}
          </Header>
          <Link to={"/"}>
            <Button label="Home" primary alignSelf="center" />
          </Link>
        </Box>
      )}
    </Box>
  );
};

export default SetStatus;
