import { onMount } from "svelte";
import { writable } from "svelte/store";

export function createWindowDimensionStore() {
    const windowDimensionStore = writable(0);

    function updateDimension() {
        windowDimensionStore.set(Math.min(window.innerWidth, window.innerHeight));
    }

    function addListener() {
        window.addEventListener("resize", updateDimension);
    }

    function removeListener() {
        window.removeEventListener("resize", updateDimension);
    }

    return {
        store: windowDimensionStore,
        addListener,
        removeListener,
    };
}
