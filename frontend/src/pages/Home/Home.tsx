import React from "react";
import { Box } from "grommet";

import Header from "../../components/Header/Header";
import Body from "../../components/Body/Body";
import Footer from "../../components/Footer/Footer";
import useWindowWidth from "../../helpers/useWindowWidth";

const Home: React.FC = (): React.ReactElement => {
  const width = useWindowWidth();

  return (
    <Box overflow={{ horizontal: "hidden" }} width={{ min: "400px" }}>
      <Header
        title="Exeter Cycling Club"
        buttonText="Upcoming Ride"
        buttonLink="/upcoming"
        width={width}
      />
      <Body width={width} />
      <Footer width={width} />
    </Box>
  );
};

export default Home;
