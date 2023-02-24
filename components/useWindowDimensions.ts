import { useEffect, useState } from "preact/hooks";
import { dimensionType } from "../src/core/dimension.ts";

export function useWindowDimensions(): dimensionType {
    const [width, setWidth] = useState(window.innerWidth);
    const [height, setHeight] = useState(window.innerHeight);

    function onWindowResize(): void {
        setWidth(window.innerWidth);
        setHeight(window.innerHeight);
    }

    useEffect(() => {
        globalThis.addEventListener("resize", onWindowResize);
        return () =>
            globalThis.removeEventListener("resize", onWindowResize);
    }, []);

    return {
        width,
        height,
    };
}
