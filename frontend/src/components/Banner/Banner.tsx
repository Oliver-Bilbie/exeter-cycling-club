import React from "react";
import { Link } from "react-router-dom";
import { useSelector } from "react-redux";
import { Box, Nav, Text } from "grommet";

import Theme from "../../theme";

const Banner: React.FC = (): React.ReactElement => {
  /* eslint-disable  @typescript-eslint/no-explicit-any */
  const user = useSelector((state: any) => {
    return state.user;
  });

  return (
    <Box
      align="center"
      as="header"
      direction="row-reverse"
      flex={false}
      gap="medium"
      justify="between"
      background={`linear-gradient(102.77deg, ${Theme.global.colors["brand"]} -9.18%, ${Theme.global.colors["text-dark"]} 209.09%)`}
      pad={{ left: "medium", right: "medium" }}
      height="75px"
      border={{ side: "bottom", color: "accent", size: "xsmall" }}
    >
      <Nav direction="row" pad="medium" color="text-strong">
        {user.id !== "" ? (
          <Text color="text-light" alignSelf="center">
            Signed in as {user.name}
          </Text>
        ) : (
          <Link to={"/signin"} style={{ textDecoration: "none" }}>
            <Text color="text-light" alignSelf="center">
              Sign in
            </Text>
          </Link>
        )}
      </Nav>
      <Link to={"/"}>
        <Box width="200px" height="75px" />
      </Link>
    </Box>
  );
};

export default Banner;
