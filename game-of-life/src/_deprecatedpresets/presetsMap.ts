import { stillLife } from "./stillLife/mod.js";
import { oscilators } from "./oscilators/mod.js";
import { spaceShips } from "./spaceShips/mod.js";
import { longLasting } from "./longLasting/mod.js";
import { machines } from "./machines/mod.js";

export const presetsMap = new Map(
    [
        ["barge", stillLife.barge],
        ["beeHive", stillLife.beeHive],
        ["biLoaf", stillLife.biLoaf],
        ["boat", stillLife.boat],
        ["circle", stillLife.circle],
        ["fishHook", stillLife.fishHook],
        ["loaf", stillLife.loaf],
        ["longBoat", stillLife.longBoat],
        ["longShip", stillLife.longShip],
        ["ship", stillLife.ship],
        ["square", stillLife.square],
        ["tub", stillLife.tub],
        ["aVerage", oscilators.aVerage],
        ["airforce", oscilators.airforce],
        ["beacon", oscilators.beacon],
        ["blinker", oscilators.blinker],
        ["oneZeroOne", oscilators.oneZeroOne],
        ["pentaDecathlon", oscilators.pentaDecathlon],
        ["pulsar", oscilators.pulsar],
        ["toad", oscilators.toad],
        ["glider", spaceShips.glider],
        ["lwss", spaceShips.lwss],
        ["mwss", spaceShips.mwss],
        ["hwss", spaceShips.hwss],
        ["acorn", longLasting.acorn],
        ["diehard", longLasting.diehard],
        ["fPentomino", longLasting.fPentomino],
        ["gliderGun", machines.gliderGun],
    ] as const,
);

type KeyOfMap<M extends Map<unknown, unknown>> = M extends
Map<infer K, unknown> ? K : never;

export type presetsKeys = KeyOfMap<typeof presetsMap>;
