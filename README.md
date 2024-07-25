# Lab1-FillingPolygon
# Michelle Mejía 2259

# Proyecto de Relleno de Polígonos

Este proyecto es un programa en Rust que dibuja y rellena varios polígonos en una imagen BMP. Incluye la funcionalidad para crear agujeros en los polígonos, utilizando un enfoque de escaneo de líneas.

## Contenido

- [Descripción](#descripción)
- [Instalación](#instalación)
- [Uso](#uso)
- [Dependencias](#dependencias)

## Descripción

Este proyecto crea una imagen BMP de 800x600 píxeles con los siguientes polígonos:

1. Una estrella amarilla.
2. Un polígono azul.
3. Un polígono rojo.
4. Un polígono verde con un agujero en forma de tetera (polígono 4 y 5).

## Instalación

1. Clona el repositorio en tu máquina local:
    ```sh
    git clone <https://github.com/michellemej22596/Lab1-FillingPolygon.git>
    ```
2. Navega al directorio del proyecto:
    ```sh
    cd polygon4
    ```
3. Compila el proyecto usando Cargo:
    ```sh
    cargo build
    ```

## Uso

Para ejecutar el programa, utiliza el siguiente comando:
```sh
cargo run
```

## Dependencias
nalgebra-glm para operaciones matemáticas y de geometría.

## Nota
Existe una rama llamada BugFixed, en donde podemos encontrar la imagen volteada. 