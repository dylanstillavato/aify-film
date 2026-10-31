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
