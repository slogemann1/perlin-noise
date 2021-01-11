import * as wasm from "perlin-noise";

function changeSeed(input) {
    let newSeed = input.value.trim();
    wasm.set_seed(newSeed);
    wasm.reset_canvas();
}

setupButton();
function setupButton() {
    let input = document.getElementById("seed");
    input.addEventListener(
        "change",
        (e) => {
            changeSeed(input);
        }
    );
}