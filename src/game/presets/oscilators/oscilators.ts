import { blinker } from "./blinker.ts";
import { toad } from "./toad.ts";
import { beacon } from "./beacon.ts";
import { pulsar } from "./pulsar.ts";
import { pentaDecathlon } from "./pentaDecathlon.ts";

export const oscilators = {
    blinker,
    toad,
    beacon,
    pulsar,
    pentaDecathlon,
} as const;
