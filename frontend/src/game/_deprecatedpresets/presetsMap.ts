import { stillLife } from "./stillLife/mod.ts";
import { oscilators } from "./oscilators/mod.ts";
import { spaceShips } from "./spaceShips/mod.ts";
import { longLasting } from "./longLasting/mod.ts";
import { machines } from "./machines/mod.ts";

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
