import React from "react";
import { Box, Text, Image, Heading } from "grommet";

interface BodyProps {
    title: string;
    text: string[];
    image: string;
    color: string;
    reverse: boolean;
    slim: boolean;
}

const BodySection: React.FC<BodyProps> = ({
    title,
    text,
    image,
    color,
    reverse,
    slim,
}): React.ReactElement => {
    return (
        <Box
            background={color}
            direction={
                slim ? "column-reverse" : reverse ? "row-reverse" : "row"
            }
            pad="large"
            gap="large"
            animation={["fadeIn", "slideUp"]}
        >
            <Box
                width={slim ? "large" : { min: "500px", max: "700px" }}
                justify="center"
                alignSelf="center"
                elevation="large"
                border={{ color: "accent", size: "large" }}
                round="small"
            >
                <Image src={image} />
            </Box>
            <Box
                direction="column"
                justify="center"
                gap="medium"
            >
                <Heading margin="small" responsive={false}>
                    {title}
                </Heading>
                {text.map(
                    (paragraph: string, index: number): React.ReactElement => {
                        return <Text key={index}>{paragraph}</Text>;
                    }
                )}
            </Box>
        </Box>
    );
};

export default BodySection;
