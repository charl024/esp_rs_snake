# Led Matrix Snake  
## Running on an ESP32, written in Rust

This simple project was a way for me to practice some Rust basics and get the hang of using it in an embedded environment. In this case, I used Espressif’s ESP32 since Rust’s ecosystem for ESP32 microcontrollers has matured significantly over the years and I had experience with programming ESP32s for other projects.  
The Led Matrix Snake takes advantage of esp-idf’s hardware-abstraction libraries to interface with the ESP32’s GPIO pins. I used Rust’s `smart_led` crate with a WS2812 LED driver to send Snake game information to an 8×8 LED matrix.

### Initial Implementation
My implementation of Snake in Rust is based on a project I did a few years ago where I had to implement Snake in Java using GUI libraries. Because of that, coding up a Rust version of Snake was essentially a process of stripping down the components abstracted by different programming languages and focusing more on the logic and the practical side of things. In this case, I adopted an object-oriented programming approach in Rust. Thankfully, Rust makes this transition pretty easy due to the nature of structs and types in the language.  
Thus, I defined a `SnakeGame` struct and implemented methods for movement logic and game rendering. In the main loop on the ESP, I call an `update` function each iteration to advance the game state.

### An Unexpected Problem
After completing the base layout for the Snake game, it was time to implement player input. However, I came across a dilemma: I didn’t have enough tactile buttons to move in all four cardinal directions. After a few minutes of thought, I decided to scrap manual input and have the snake move automatically toward an apple on the grid. If we follow all the rules stated in Snake, this essentially becomes an automatic Snake solver. Of course, this was pretty difficult. The problem reduces to finding a maximum-length simple path from one cell to another, visiting each cell exactly once. As it turns out, this is a Hamiltonian path problem, which is NP-hard (see https://en.wikipedia.org/wiki/Hamiltonian_path_problem).

### The Solution
So, how does one get around this? It’s simple – ignore the rules of Snake. What if the snake crosses over itself? It just won’t die. Now we have a pseudo-Snake game.  
This simplifies the problem a lot, so I coded an automated movement algorithm.

1. Precomputes a path from the snake’s head to the apple.  
2. Pushes each target cell’s coordinates onto a stack.  
3. Pops from the stack to move the snake one step at a time.  

When the stack is empty, the snake reaches the apple, a new apple spawns, and I recompute the path. I also made the snake change color randomly every five apples for a bit of flair.

### Conclusion
I started and finished this exercise back in December 2024, but I never got around to documenting it. Overall, it was a great learning experience. I figured out how to make and use data structures in Rust—stacks and queues are easy, and structs are similar to C/C++, which made the transition comfortable. Rust’s borrow checker was certainly something new, and I can see why Rust is advertised as “very safe.”  
In the future, I can improve this Snake project by implementing a pathfinder that prevents the snake from crossing over itself. That would require tackling the Hamiltonian path problem head-on, so it’ll be a fun challenge when the time comes.
