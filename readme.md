# Ray Casting Sandbox

## An implementation of raycasting to display a pseudo 3D playground

Many early "3D" games (such as Wolfenstein 3D) used the same or similar techniques to create pseudo 3D worlds.
Although only squares can be placed in the editor this engine works with any polygon or shape. (diagonal walls aswell).

## About

This ray casting engine was written in rust and uses the macroquad game development library for drawing to the screen. It was very fun to work on and I will update it to be a complete resource for anyone trying to make one themselves.

## Highlights

- The engine corrects for all distortion and has toggles between different distance calculation methods and ray casting methods.
- When in debug/settings mode there is a visualization of what the camera sees vs the rays being cast out.
- Almost everything from the FOV to Amount of rays being cast is adjustable live.
- There is a few different textures to select through and they can be placed freely.

# Try it out

The engine was built for wasm and is available [HERE](https://raycasting.nathanferns.xyz/)

## Screenshots

![pic1](https://cdn.discordapp.com/attachments/869367514008793128/875152315353542737/unknown.png)
![pic2](https://cdn.discordapp.com/attachments/869367514008793128/875152352842227793/unknown.png)
