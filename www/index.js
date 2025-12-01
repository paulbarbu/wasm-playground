

import {memory} from "wasm-game-of-life/wasm_game_of_life_bg.wasm";
import * as wasm from "wasm-game-of-life";

const w = 10;
const h = 10;
let u = wasm.Universe.new(w, h);
// let u = wasm.Universe.rand(w, h);
let pre = document.getElementById("text-canvas");

// since the tutorial's 
// import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
// does not work, I have used:
// https://github.com/wasm-tool/wasm-pack-plugin/issues/72
console.log("memory")
console.log(memory)
console.log("wasm")
console.log(wasm)
console.log("wasm_memory()")
console.log(wasm.wasm_memory())
// console.log(wasm.wasm_memory())

function getBitForRowCol(mem, row, col)
{
    const byte = Math.floor((row*w + col)/8);
    const bit_index = (row*w + col)%8;
    const mask = (1 << bit_index);
    return (mem[byte] & mask) === mask; //mask off where my bit is and get it from the byte
}

function render()
{    
    // pre.textContent = u.render();
    pre.textContent = "";
    let mem_view = new Uint8Array(memory.buffer, u.cells(), (w*h)/8);
    for(let r = 0; r<h; r++)
    {
        for(let c = 0; c<w; c++)
        {
            // pre.textContent += mem_view[r*w+c] + " ";
            pre.textContent += (getBitForRowCol(mem_view, r, c) ? "1" : "0") + " ";
        }
        pre.textContent += "\n";
    }

    u.tick();

    setTimeout(() => requestAnimationFrame(render), 500);
}

requestAnimationFrame(render);
