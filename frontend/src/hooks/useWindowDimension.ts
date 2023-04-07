import { useEffect, useState } from "react";

export function useWindowDimension(): number {
    const [dimension, setDimension] = useState(0);

    useEffect(() => {
        function onWindowResize(): void {
            setDimension(
                Math.min(window.innerWidth, window.innerHeight),
            );
        }

        onWindowResize();
        globalThis.addEventListener("resize", onWindowResize);
        return () => {
            globalThis.removeEventListener("resize", onWindowResize);
        };
    }, []);

    return dimension;
}
