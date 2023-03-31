import { Grommet } from "grommet";
import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Provider } from "react-redux";

import Home from "./pages/Home/Home";
import RidePage from "./pages/RidePage/RidePage";
import RouteSelect from "./pages/Select/Select";
import RouteCancel from "./pages/Cancel/Cancel";
import Contact from "./pages/Contact/Contact";
import SignIn from "./pages/SignIn/SignIn";
import Redirect from "./pages/Redirect/Redirect";
import Unsubscribe from "./pages/Unsubscribe/Unsubscribe";
import ErrorPage from "./error-page";

import { store } from "./redux/store";
import Theme from "./theme";

const router = createBrowserRouter([
    {
        path: "/",
        element: <Home />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/upcoming",
        element: <RidePage />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/select",
        element: <RouteSelect />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/cancel",
        element: <RouteCancel />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/contact",
        element: <Contact />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/signin/*",
        element: <SignIn />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/redirect/*",
        element: <Redirect />,
        errorElement: <ErrorPage />,
    },
    {
        path: "/unsubscribe/*",
        element: <Unsubscribe />,
        errorElement: <ErrorPage />,
    },
]);

const root = ReactDOM.createRoot(
    document.getElementById("root") as HTMLElement
);

root.render(
    // <React.StrictMode>
    <Provider store={store}>
        <Grommet theme={Theme} full={true}>
            <RouterProvider router={router} />
        </Grommet>
    </Provider>
    // </React.StrictMode>
);
