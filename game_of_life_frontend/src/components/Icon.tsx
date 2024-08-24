import type { ReactElement } from "react";

type Props = {
    readonly icon: "next" | "pause" | "play";
};

export function Next(): ReactElement {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            className="w-4"
        >
            <path
                fill="white"
                d="M84.41 106c-15.63.1-27.67 13.8-25.69 29.3 16 124 16 117.4 0 241.4-2.54 19.8 17.33 35 35.79 27.3L361.5 292.9v98.8c0 7.9 8.9 14.2 20 14.3h52c11.1-.1 20-6.4 20-14.3V120.2c-.1-7.8-9-14.1-20-14.2h-52c-11 .1-19.9 6.4-20 14.2v98.9L94.51 108c-3.2-1.3-6.63-2-10.1-2z"
            />
        </svg>
    );
}

export function Pause(): ReactElement {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            className="w-4"
        >
            <path
                fill="white"
                d="M120.16 45A20.162 20.162 0 0 0 100 65.16v381.68A20.162 20.162 0 0 0 120.16 467h65.68A20.162 20.162 0 0 0 206 446.84V65.16A20.162 20.162 0 0 0 185.84 45h-65.68zm206 0A20.162 20.162 0 0 0 306 65.16v381.68A20.162 20.162 0 0 0 326.16 467h65.68A20.162 20.162 0 0 0 412 446.84V65.16A20.162 20.162 0 0 0 391.84 45h-65.68z"
            />
        </svg>
    );
}

export function Play(): ReactElement {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            className="w-4"
        >
            <path
                fill="white"
                d="M106.854 106.002a26.003 26.003 0 0 0-25.64 29.326c16 124 16 117.344 0 241.344a26.003 26.003 0 0 0 35.776 27.332l298-124a26.003 26.003 0 0 0 0-48.008l-298-124a26.003 26.003 0 0 0-10.136-1.994z"
            />
        </svg>
    );
}

export function Icon({ icon }: Props): ReactElement {
    switch (icon) {
        case "next":
            return <Next />;
        case "pause":
            return <Pause />;
        case "play":
            return <Play />;
    }
}
