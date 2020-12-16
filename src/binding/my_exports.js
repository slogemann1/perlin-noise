var canvasObjects = [];

export function create_h2(str) {
    let body = document.getElementsByTagName("body")[0];
    let h1 = document.createElement("h2");
    h1.innerHTML = str;
    body.appendChild(h1);
}

export function console_log(str) {
    console.log(str);
}

export function new_canvas(str, width, height) {
    let canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    let context = canvas.getContext("2d");

    document.getElementsByTagName("body")[0].appendChild(canvas);

    canvasObjects.push({
        name: str,
        canvas: canvas,
        context: context,
        width: width,
        height: height
    });
}

export function flush_canvas(str, data) {
    canvasObjects.forEach((obj) => {
        if(obj.name === str) {
            let arr = Uint8ClampedArray.from(data);
            let iData = new ImageData(arr, obj.width, obj.height);
            obj.context.putImageData(iData, 0, 0);
        }
    });
}