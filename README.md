<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ai.storyteller.filmcraft.png" width="112" height="112" alt="Icono de @Film">
</p>

<h1 align="center">@Film</h1>

<p align="center"><b>El editor de vídeo gratis para gestores de redes, en tu navegador.</b><br>
De <a href="https://www.useaify.app">Aify</a>. Lo maneja tu propia IA.</p>

<p align="center">
  <a href="https://film.useaify.app">film.useaify.app</a> ·
  <a href="https://github.com/storytold/filmcraft">Basado en FilmCraft</a>
</p>

@Film es un **fork de [FilmCraft](https://github.com/storytold/filmcraft)**, el editor de vídeo
no lineal en Rust puro del equipo de ArtCraft. Va en el navegador con WebAssembly y en escritorio
(macOS, Windows y Linux), con timeline profesional, color, audio, subtítulos y exportación H.264.

Lo que añade Aify:

- **Español por defecto** y la familia Aify: @Social (programar), @Pictures (imagen) y @Gen.
- **Pensado para reels:** formato 9:16, subtítulos automáticos, cortes de silencios y exportación para Instagram y TikTok (en camino, ver el plan).
- **Tu IA:** tu Claude o tu ChatGPT manejan el editor con los comandos de FilmCraft por el conector de Aify, o con tu propia clave. Aify nunca paga ni guarda tu IA.

El plan está en [aify-social/docs/aify-film](https://github.com/dylanstillavato/aify-social/tree/main/docs/aify-film).
El README original de FilmCraft está en [docs/README-filmcraft.md](docs/README-filmcraft.md).

## Compilar

Igual que FilmCraft: `cargo run -p filmcraft` para escritorio y `packaging/web/package.sh` para la web
(ver [docs/web.md](docs/web.md)).

## Licencia y créditos

Como FilmCraft, @Film tiene licencia [MIT](LICENSE-MIT) o [Apache-2.0](LICENSE-APACHE), a elegir.
FilmCraft es obra del ArtCraft Team y de los colaboradores de FilmCraft. @Film es un fork
independiente: no está afiliado al ArtCraft Team ni lo respalda. El nombre y las marcas de ArtCraft
se han retirado de este fork, como piden sus condiciones. Ver [NOTICE](NOTICE) y [ATTRIBUTION.md](ATTRIBUTION.md).

<sub>Adobe y Premiere Pro son marcas de Adobe Inc. @Film no está afiliado a Adobe ni respaldado por Adobe.</sub>
