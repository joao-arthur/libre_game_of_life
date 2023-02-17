import { useEffect, useState } from "preact/hooks";

type dimensionType = {
    readonly width: number;
    readonly height: number;
};

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
