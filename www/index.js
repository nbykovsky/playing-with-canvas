import * as wasm from "wasm-package";
import { memory } from "wasm-package/playing_with_canvas_bg";


const canvas = document.getElementById("playing-canvas");
const HEIGNT = 800
const WIDTH = 1200
const K = 100000;


const BORDER_COLOR = "#F0000F"
const FILL_COLOR = "#00F00F"

canvas.height = HEIGNT
canvas.width = WIDTH

const ctx = canvas.getContext('2d');


function draw_rect(rect) {
    
    const size = rect.size();
    const pointsPtr = rect.points();
    const points = new Uint32Array(memory.buffer, pointsPtr, size * 2);


    ctx.strokeStyle = BORDER_COLOR;
    ctx.fillStyle = FILL_COLOR;

    ctx.beginPath();
    ctx.moveTo(points[0]/K, points[1]/K);

    for(let idx = 2; idx < size*2;idx+=2) {
        const x = points[idx]/K;
        const y = points[idx+1]/K;
        ctx.lineTo(x, y);
    }

    ctx.closePath();
    ctx.fill();
    ctx.stroke();

}


var rect = wasm.Rect.new();
rect.push(wasm.Point.new(200*K,200*K));
rect.push(wasm.Point.new(300*K,200*K));
rect.push(wasm.Point.new(300*K,300*K));
rect.push(wasm.Point.new(200*K,300*K));


const renderLoop = () => {

    ctx.clearRect(0, 0, WIDTH, HEIGNT);

    draw_rect(rect);
    rect.rotate(wasm.Point.new(310*K, 310*K), 0.01);

    requestAnimationFrame(renderLoop);
  };

requestAnimationFrame(renderLoop);

