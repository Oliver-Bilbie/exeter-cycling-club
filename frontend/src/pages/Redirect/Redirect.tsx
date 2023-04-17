import React, { useEffect, useState } from "react";
import { useSearchParams, Navigate } from "react-router-dom";
import { useDispatch } from "react-redux";
import { Box } from "grommet";

import LoadingSpinner from "../../components/LoadingSpinner/LoadingSpinner";
import { callApi } from "../../helpers/callApi";
import { setData } from "../../redux/user";
import { UserType } from "../../types";
import { AUTH_ENDPOINT } from "../../constants/endpoints";

const Redirect: React.FC = (): React.ReactElement => {
  const [redirect, setRedirect] = useState(false);

  const dispatch = useDispatch();

  // Extract request code from query parameters
  const [authParams] = useSearchParams();
  const authCode = authParams.get("code") as string;

  const handleResponse = (response): void => {
    if (response.body.status === 200) {
      const body = response.body.body;
      const newUserData: UserType = {
        id: body.id,
        name: body.name,
        access_token: body.access_token,
        admin: body.admin,
      };
      dispatch(setData(newUserData));
    }
    setRedirect(true);
  };

  useEffect(() => {
    // Request auth token from Strava.
    // This is done via our backend to avoid exposing the secret key.
    callApi("GET", `${AUTH_ENDPOINT}/${authCode}`, handleResponse);
  }, []);

  return (
    <Box background="background" align="center" justify="center" fill>
      {redirect ? <Navigate replace to="/" /> : <LoadingSpinner size="200px" />}
    </Box>
  );
};

export default Redirect;
