import { useEffect, useMemo, useState } from "preact/hooks";
import { dimensionType } from "../src/core/dimension.ts";

export function useWindowDimensions(): dimensionType {
    const [width, setWidth] = useState(window.innerWidth);
    const [height, setHeight] = useState(window.innerHeight);
    const dimensions = useMemo(() => ({ width, height }), [
        width,
        height,
    ]);

    function onWindowResize(): void {
        setWidth(window.innerWidth);
        setHeight(window.innerHeight);
    }

    useEffect(() => {
        globalThis.addEventListener("resize", onWindowResize);
        return () =>
            globalThis.removeEventListener("resize", onWindowResize);
    }, []);

    return dimensions;
}
