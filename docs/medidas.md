# @Film: medidas con brutos reales (2026-10-31)

Encargo 5 del plan (`aify-social/docs/aify-film/plan.md`). Web compilada desde `aify/tema-aify` (`cargo xtask web`, wasm-release con LTO thin), servida con `vite preview` desde `aify-film-web` y abierta en Chrome en un MacBook (macOS 13, Intel). Se midió con `window.filmcraft`: `importUrl`, `file.newSequence` desde el clip, `ui.playback` y los contadores de `filmcraft.info().webcodecsStats`.

| | Bruto de cámara (Sony) | Vídeo de móvil |
|---|---|---|
| Archivo | H.264 High 4:2:0 8 bits, 3840×2160, 23,976 fps, 100 Mb/s, 110 MB | HEVC Main, 1440×2560 (vertical), 30 fps, 15,6 Mb/s, 78 MB |
| Importar (`importUrl`) | 2,1 s | 2,9 s |
| Fotogramas decodificados al reproducir | 5,5-9,6 por segundo | 30,5 por segundo |
| Fotogramas de la interfaz | 3,4 por segundo | 26,9 por segundo |
| Fallos de WebCodecs | 0 | 0 |

Estado del navegador: WebGPU (`BrowserWebGpu`), compositor en GPU, WebCodecs con avc, hevc, vp9 y av1, OPFS, sin aislamiento entre orígenes (`crossOriginIsolated: false`, un solo hilo). Arranque: wasm compilado en 0,44 s y app lista en 0,48 s. Memoria JS: 244 MB tras importar los dos.

## Lo que limita el 4K

El decodificador va bien; lo caro es copiar cada fotograma a la memoria de WASM. En 5 s de reproducción del bruto Sony: 48 fotogramas decodificados, todos por la vía «native» (`VideoFrame.copyTo`), que tardó 1094 ms en total, **unos 23 ms por fotograma 4K**. Como todo va en un solo hilo, esa copia más la composición no caben en los 41 ms de un fotograma a 24 fps. El vertical del móvil (3,7 Mpx frente a 8,3 Mpx) sí cabe.

## Siguiente paso (encargo 5b)

Leer los fotogramas a la resolución de reproducción del monitor (1/2 o 1/4) en vez de a 4K completo: dibujar el `VideoFrame` escalado en un `OffscreenCanvas` (la vía «canvas» ya existe en `apps/filmcraft-web/src/webcodecs.rs`) y copiar solo eso. Exportar sigue leyendo a resolución completa. Alternativa si no basta: proxies a 720p al importar.

Los brutos 4K de cámara son el material típico de SOLD OUT, así que esto va antes que el espacio «Reel».

## Probado y descartado: leer el fotograma reducido (2026-10-31, issue #6)

Idea: cuando el monitor va a 1/2 o 1/4, dibujar el `VideoFrame` reducido y leer solo eso. En el mismo Mac y con el mismo bruto Sony, mediana de 8 fotogramas, medido en la página:

| Forma de leer un fotograma 4K | ms por fotograma |
|---|---|
| `copyTo` del fotograma entero (NV12, lo que hace hoy) | 22,8 |
| `createImageBitmap` reducido a 1/4 + `copyTo` RGBA | 32,8 |
| `createImageBitmap` reducido a 1/2 + `copyTo` RGBA | 54,0 |
| `OffscreenCanvas` 2D a 1/4 + `getImageData` | 84,0 |
| `OffscreenCanvas` 2D a 1/2 + `getImageData` | 115,8 |

Con la vía del canvas integrada en `webcodecs.rs`, la reproducción bajó a 7,5 fotogramas decodificados por segundo (75 ms por lectura). Reducir en la GPU y leer después cuesta más que leer el NV12 entero: la espera a la GPU domina. Se ha deshecho.

**Lo que haría falta:** no leer el fotograma a la CPU. Con el compositor en WebGPU, el `VideoFrame` puede ir directo a una textura (`GPUQueue.copyExternalImageToTexture` o `GPUDevice.importExternalTexture`) y componerse ahí. El frame server y el compositor trabajan hoy con fotogramas en memoria, así que es un cambio de diseño en `crates/render` y en `apps/filmcraft-web`, no un encargo de una noche. Mientras tanto, para brutos 4K: proxies (hoy dan error en la web porque necesitan un hilo) o exportar desde la cámara a 1080p.
