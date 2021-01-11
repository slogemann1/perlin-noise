# Eine Implementation von Perlin Noise geschrieben für die Mini-Facharbeit
Diese Implementation wurde in Rust geschrieben und zu WebAssembly kompiliert um als
interaktives Beispiel im Browser zu dienen. Diesen findet man unter: https://slogemann1.github.io/Perlin-Noise/index.html

## Dateien Erstellen
Für das Kompilieren der Dateien werden "Cargo" und "Wasm-pack" benötigt.
Für das spätere ausführen wird "Node" benutzt.

Folgende Befehle werden für die erste Kompilation verwendet:
```
wasm-pack build
npm init wasm-app www
cd www
rm ./index.html ./index.js ./package.json
cp ../src/binding/index.html ./index.html
cp ../src/binding/package.json ./package.json
cp ../src/binding/index.js ./index.js
npm install
npm run start
```