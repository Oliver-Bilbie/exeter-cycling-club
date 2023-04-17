import React from "react";
import { Link } from "react-router-dom";
import { Box, Image, Carousel, Stack, Button, Heading } from "grommet";

interface ShowCaseProps {
  mainText: string;
  buttonText?: string;
  buttonLink?: string;
  width: number;
  compact?: boolean;
}

const Showcase: React.FC<ShowCaseProps> = ({
  mainText,
  buttonText,
  buttonLink,
  width,
  compact,
}): React.ReactElement => {
  return (
    <Box height={compact ? "small" : "medium"}>
      <Stack fill interactiveChild="last">
        <Carousel fill controls={false} play={10000}>
          <Image src={require("../../images/header1.jpg")} fit="cover" />
          <Image src={require("../../images/header2.jpg")} fit="cover" />
          <Image src={require("../../images/header3.jpg")} fit="cover" />
        </Carousel>
        <Box width={{ min: "1000px" }} fill>
          <Image
            fill
            src={require("../../images/overlay.png")}
            fit="cover"
            alignSelf="end"
          />
        </Box>
        <Box
          width={width > 930 ? `${width / 3}px` : "280px"}
          margin={{
            top: "none",
            bottom: "medium",
            left: "medium",
            right: "medium",
          }}
          gap="xsmall"
        >
          <Box width="200px" height="125px">
            <Link to={"/"}>
              <Box width="200px" height="125px" />
            </Link>
          </Box>
          <Heading color="text-light" weight="lighter" responsive={false}>
            {mainText}
          </Heading>
          {buttonText && buttonLink && (
            <Box width="small">
              <Link to={buttonLink}>
                <Button label={buttonText} color="brand" primary />
              </Link>
            </Box>
          )}
        </Box>
      </Stack>
    </Box>
  );
};

export default Showcase;
