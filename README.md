# mod1

mod1 is a simple water simulation demo written in rust by [gsmith](https://github.com/guilhemSmith).  

It use the pipe model to store the pression between each cells of the water heigthmap.  
The water height and pressure evolution run on the cpu so the map has a resolution of only 100x100 cells.  

<center>

![waves](media/waves.gif)

</center>

## Compilation from sources
### On macOs and Linux
* make sure to have rust installed: https://www.rust-lang.org/tools/install 
* `cargo build --release` to build the demo.

<center>

![rain](media/rain.gif)

</center>

## Usage  
### Launch
`cargo run --release -- <mapfile>` or `./target/release/mod1 <mapfile>`, will launch the demo with the specified mapfile.  

_example: `cargo run --release -- resources/demo1.mod1`_    

There are multiple map file in the resources folder, you can edit them or make your own.  
A map file should contain only one point per line, each point corresponding to 3 float number separated by spaces.

### controls
<center>

| input                 | effect                                  |
| --------------------- | --------------------------------------- |
| <kbd>left-click</kbd> | move the camera around                  |
| <kbd>w</kbd>          | add water on the side of the map (wave) |
| <kbd>t</kbd>          | add water on low level terrain (tide)   |
| <kbd>r</kbd>          | add rain                                |
| <kbd>d</kbd>          | drain water on low level terrain        |
| <kbd>+</kbd>          | increase strength of water command      |
| <kbd>-</kbd>          | decrease strength of water command      |

![tide](media/tide.gif)

</center>
