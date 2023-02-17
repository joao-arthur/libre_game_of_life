import { status } from "./status.ts";

export function iteration(
    currentStatus: status,
    numberOfAliveNeighbors: 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8,
): status {
    switch (currentStatus) {
        case status.ALIVE:
            return [2, 3].includes(numberOfAliveNeighbors)
                ? status.ALIVE
                : status.DEAD;
        case status.DEAD:
            return numberOfAliveNeighbors === 3
                ? status.ALIVE
                : status.DEAD;
    }
}
