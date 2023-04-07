import { useEffect, useState } from "react";

export function useWindowDimension(): number {
    const [dimension, setDimension] = useState(getDimension());

    function onWindowResize(): void {
        setDimension(getDimension());
    }

    function getDimension(): number {
        if(globalThis.window === undefined) {
            return 0;
        }
        return Math.min(window.innerWidth, window.innerHeight);
    }

    useEffect(() => {
        globalThis.addEventListener("resize", onWindowResize);
        return () =>
            globalThis.removeEventListener("resize", onWindowResize);
    }, []);

    return dimension;
}
