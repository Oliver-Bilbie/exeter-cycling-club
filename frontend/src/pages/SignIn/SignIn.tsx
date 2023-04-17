import React from "react";
import { Box, Image, Button, Text } from "grommet";

import Header from "../../components/Header/Header";
import Footer from "../../components/Footer/Footer";
import { handleLogin } from "../../helpers/handleLogin";
import useWindowWidth from "../../helpers/useWindowWidth";

const SignIn: React.FC = (): React.ReactElement => {
  const width = useWindowWidth();

  return (
    <Box
      background="background"
      width={{ min: "400px" }}
      overflow={{ horizontal: "hidden" }}
      fill
    >
      <Header title="Sign In" buttonText="Home" buttonLink="/" width={width} />
      <Box
        justify="center"
        align="center"
        height={{ min: "400px" }}
        gap="medium"
        fill
      >
        <Text weight="bold">Sign in to set this week&#39;s route.</Text>
        <Text textAlign="center">
          There are currently no benefits to signing in for non-admins, however
          this may change in the future. Watch this space!
        </Text>
        <Button onClick={handleLogin} alignSelf="center">
          <Image
            src={require("../../images/btn_strava_connectwith_orange.png")}
            fit="contain"
          />
        </Button>
      </Box>
      <Box fill />
      <Footer width={width} />
    </Box>
  );
};

export default SignIn;
