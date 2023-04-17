import React, { useState } from "react";
import {
  Box,
  Button,
  Form,
  TextInput,
  TextArea,
  FormField,
  Heading,
} from "grommet";

import Header from "../../components/Header/Header";
import Footer from "../../components/Footer/Footer";
import LoadingSpinner from "../../components/LoadingSpinner/LoadingSpinner";
import { callApi } from "../../helpers/callApi";
import useWindowWidth from "../../helpers/useWindowWidth";
import { CONTACT_ENDPOINT } from "../../constants/endpoints";

enum STATUS {
  "IDLE",
  "LOADING",
  "SUCCESS",
  "FAILED",
}

const Contact: React.FC = (): React.ReactElement => {
  const width = useWindowWidth();

  const [email, setEmail] = useState("");
  const [message, setMessage] = useState("");
  const [formStatus, setFormStatus] = useState(STATUS.IDLE);

  const handleResponse = (response): void => {
    setFormStatus(
      response.body.status === 200 ? STATUS.SUCCESS : STATUS.FAILED
    );
  };

  const handleSubmit = (): void => {
    setFormStatus(STATUS.LOADING);
    callApi(
      "POST",
      CONTACT_ENDPOINT,
      handleResponse,
      undefined,
      undefined,
      `{"contact_email": "${email}", "message": "${message}"}`
    );
  };

  return (
    <Box
      background="background"
      width={{ min: "400px" }}
      overflow={{ horizontal: "hidden" }}
      fill
    >
      <Header
        title="Contact Us"
        buttonText="Home"
        buttonLink="/"
        width={width}
      />
      <Box justify="center" height={{ min: "600px" }} fill pad="medium">
        <Box
          alignSelf="center"
          align="center"
          justify="center"
          pad="large"
          animation={["fadeIn", "slideUp"]}
        >
          {formStatus === STATUS.SUCCESS ? (
            <Heading>Message sent successfully</Heading>
          ) : (
            <Form onSubmit={handleSubmit}>
              <FormField
                name="email"
                label="Contact Email"
                required
                validate={{
                  regexp:
                    /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/,
                  message: "invalid",
                  status: "error",
                }}
              >
                <TextInput
                  name="email"
                  value={email}
                  onChange={(event): void => setEmail(event.target.value)}
                  plain={false}
                />
              </FormField>
              <FormField name="message" label="Message" required>
                <Box height="200px">
                  <TextArea
                    name="message"
                    value={message}
                    onChange={(event): void => setMessage(event.target.value)}
                    plain={false}
                    resize={false}
                    fill
                  />
                </Box>
              </FormField>
              <Box alignSelf="center" align="center" margin={{ top: "40px" }}>
                {formStatus === STATUS.LOADING ? (
                  <LoadingSpinner size="36px" />
                ) : (
                  <Button label="Send" color="brand" primary type="submit" />
                )}
              </Box>
            </Form>
          )}
        </Box>
      </Box>
      <Box fill />
      <Footer width={width} />
    </Box>
  );
};

export default Contact;
