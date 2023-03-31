// Credit to user "QoP" from StackOverflow for this hook

import { useState, useEffect } from "react";

function getWindowWidth(): number {
    const { innerWidth: width } = window;
    return width;
}

export default function useWindowWidth(): number {
    const [windowWidth, setWindowWidth] = useState(getWindowWidth());

    useEffect((): (() => void) => {
        function handleResize(): void {
            setWindowWidth(getWindowWidth());
        }

        window.addEventListener("resize", handleResize);
        return (): void => window.removeEventListener("resize", handleResize);
    }, []);

    return windowWidth;
}
