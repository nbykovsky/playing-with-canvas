import * as wasm from "wasm-package";
import { memory } from "wasm-package/playing_with_canvas_bg";


const canvas = document.getElementById("playing-canvas");
const HEIGNT = 400
const WIDTH = 600

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
    ctx.moveTo(points[0], points[1]);

    for(let idx = 2; idx < size*2;idx+=2) {
        const x = points[idx];
        const y = points[idx+1];
        ctx.lineTo(x, y);
    }

    ctx.closePath();
    ctx.fill();
    ctx.stroke();

}

const rect = wasm.Rect.new();
rect.push(wasm.Point.new(0,0));
rect.push(wasm.Point.new(100,0));
rect.push(wasm.Point.new(50,100));

draw_rect(rect);



