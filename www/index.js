import * as wasm from "wasm-game-of-life";

let u = wasm.Universe.new(10, 10);
let pre = document.getElementById("text-canvas");

function render()
{
    pre.textContent = u.render();
    u.tick();

    setTimeout(() => requestAnimationFrame(render), 500);
}

requestAnimationFrame(render);
