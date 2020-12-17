import * as wasm from "perlin-noise";

setInterval(wasm.animate_callback, 10);

function changeSeed(input) {
    let newSeed = input.value.trim();
    wasm.set_seed(newSeed);
    wasm.reset_canvas();
}

setupButton();
function setupButton() {
    let input = document.getElementsById("seed");
    input.addEventListener(
        "change",
        (e) => {
            changeSeed(input);
        }
    );
}