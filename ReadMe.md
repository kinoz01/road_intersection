## Road Intersection (Macroquad)
A tiny traffic intersection sim built with [Macroquad]. Cars spawn from the four roads and obey a simple traffic-light cycle. 

## Controls
- `↑`: spawn from bottom (moving up)
- `↓`: spawn from top (moving down)
- `→`: spawn from left (moving right)
- `←`: spawn from right (moving left)
- `R`: spawn random direction
- `Esc`: quit

## Spawned Cars Directions
- `blue` → straight
- `yellow `→ turn left
- `purple` → turn right

## How it works (quick tour)
- `lights.rs`: TrafficLights cycles the active direction: all-red between 3s greens.

- `car.rs`: Car holds position, direction, color, turning rules and spacing checks.

- `main.rs`: Renders the scene (roads, crosswalks, lights), spawns cars on keypress, updates cars each frame, and draws them (sprites or rectangles).