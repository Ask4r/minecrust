import { run } from "world";
import "./style.scss";
import { useState } from "react";

export default function DisplayWorld() {
    const [isRunning, setIsRunning] = useState(false);

    const runWorld = () => {
        if (isRunning) { return; }
        setIsRunning(true);
        run("#minecrust-world-canvas");
    };

    return (
        <div className="wrapper" onClick={runWorld}>
            <canvas
                id="minecrust-world-canvas"
                className="canvas"
            >Canvas is not supported</canvas>
            <p className={`hint ${isRunning ? "hidden" : ""}`}>Click on screen and wait for loading</p>
        </div>
    );
}
