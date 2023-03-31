import { ResponseType } from "../types";

export const callApi = (
    method: "GET" | "POST" | "PUT" | "DELETE",
    endpoint: string,
    onLoad: (response: ResponseType) => void,
    token?: string,
    noCache?: boolean,
    body?
): void => {
    const request = new XMLHttpRequest();

    let response = {};

    request.onerror = (): void => {
        response = { message: "An error has occurred" };
    };
    request.ontimeout = (): void => {
        response = { message: "The request has timed out" };
    };
    request.onload = (): void => {
        if (request.response.errors) {
            response = {
                status: 500,
                body: `${request.response.errors[0].code} ${request.response.errors[0].field}`,
            };
        } else {
            response = {
                status: 200,
                body: request.response,
            };
        }
    };
    request.timeout = 15000;
    request.responseType = "json";
    request.addEventListener("load", (): void => {
        onLoad(response);
    });
    request.addEventListener("timeout", (): void => {
        onLoad(response);
    });
    request.addEventListener("error", (): void => {
        onLoad(response);
    });
    request.open(method, endpoint);
    if (token) {
        request.setRequestHeader("Authorization", `Bearer ${token}`);
    }
    if (noCache) {
        request.setRequestHeader("Cache-Control", "no-cache");
    }
    request.send(body);
};
