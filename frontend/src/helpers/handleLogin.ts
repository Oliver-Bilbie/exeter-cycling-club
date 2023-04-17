import { STRAVA_AUTH_ENDPOINT, STRAVA_CLIENT_ID } from "../constants/endpoints";

export const handleLogin = (): void => {
  const redirectUrl = `${process.env.REACT_APP_URL}/redirect`;
  const scope = "read";
  window.location =
    `${STRAVA_AUTH_ENDPOINT}?client_id=${STRAVA_CLIENT_ID}&response_type=code&redirect_uri=${redirectUrl}/exchange_token&approval_prompt=force&scope=${scope}` as string &
      Location;
};
