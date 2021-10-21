import * as wasm from "wasm-package";
import { memory } from "wasm-package/playing_with_canvas_bg";


const canvas = document.getElementById("playing-canvas");
const HEIGNT = 800
const WIDTH = 1200

const BORDER_COLOR = "#F0000F"
const FILL_COLOR = "#00F00F"

canvas.height = HEIGNT
canvas.width = WIDTH

const ctx = canvas.getContext('2d');

ctx.strokeStyle = BORDER_COLOR;
ctx.fillStyle = FILL_COLOR;

function draw_scene(scene) {
    const bufPtr = scene.render();
    const oneElelemtArray = new Int32Array(memory.buffer, bufPtr, 1);
    const bufSize = oneElelemtArray[0];
    var buf = new Int32Array(memory.buffer, bufPtr, bufSize);

    var idx = 1;
    while (idx < buf.length) {
        var num_remaining = buf[idx++];

        if (num_remaining == 0) {
            continue;
        }
        
        ctx.beginPath();

        const x = buf[idx++];
        const y = buf[idx++];
        num_remaining -= 2;

        ctx.moveTo(x, y);
        while (num_remaining > 0) {
            const x = buf[idx++];
            const y = buf[idx++];

            ctx.lineTo(x, y);

            num_remaining -= 2;
        }

        ctx.closePath();
        ctx.fill();
        ctx.stroke();
    }

}

var scene = wasm.Scene.new();

const renderLoop = () => {

    ctx.clearRect(0, 0, WIDTH, HEIGNT);

    draw_scene(scene);
    scene.tick();

    requestAnimationFrame(renderLoop);
  };

requestAnimationFrame(renderLoop);

