export function create_h2(str) {
    let body = document.getElementsByTagName("body")[0];
    let h1 = document.createElement("h2");
    h1.innerHTML = str;
    body.appendChild(h1);
}

export function console_log(str) {
    console.log(str);
}