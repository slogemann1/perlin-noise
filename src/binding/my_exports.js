var canvasObjects = [];

export function console_log(str) {
    console.log(str);
}

export function set_style(id, style) {
    document.getElementById(id).style = style;
}

export function new_canvas(str, width, height) {
    let div = document.createElement("div");
    div.id = str;

    let h2 = document.createElement("h2");
    h2.id = str + "_h2";
    div.appendChild(h2);

    let canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    canvas.id = str + "_canvas";
    let context = canvas.getContext("2d");

    div.appendChild(canvas);
    document.getElementsByTagName("body")[0].appendChild(div);

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

export function set_title(canvas_id, title) {
    document.getElementById(canvas_id + "_h2").innerHTML = title;
}