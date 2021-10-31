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
    const numTriangles = oneElelemtArray[0];
    var buf = new Int32Array(memory.buffer, bufPtr, numTriangles*6+1);

    for(var idx = 0; idx < numTriangles; idx++) {
        ctx.beginPath();
        for(var jdx = 0; jdx < 3; jdx++) {
            const x_idx = idx*6+1 + jdx*2;
            const y_idx = x_idx + 1;
            const x = buf[x_idx];
            const y = buf[y_idx];

            if (jdx == 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
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

