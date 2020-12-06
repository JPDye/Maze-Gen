# Maze-Gen

Generate Mazes using a number of different algorithms. Colour them, animate them and display them as ASCII art, a png or a gif.

  <p align="center">
    <img src="https://github.com/JPDye/Maze-Gen/blob/main/imgs/colourful.png" />
  </p>

## Examples
<details><summary>Hunt and Kill Algorithm</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Maze-Gen/blob/main/imgs/colourful_hunt_and_kill.png" />
  </p>
  <p>Pick a random cell and perform a mostly random walk (avoid already visited cells). When a cell is reached where there are no unvisited neighbours, end the walk and start a new one at the first unvisited cell that borders a visited cell. Creates mazes with long, windy passages. Not the most efficient algorithm.</p>
</details>

<details><summary>Wilson's Algorithm</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Maze-Gen/blob/main/imgs/colourful_wilsons.png" />
  </p>
  <p>Loop-erased random walk. Creates mazes with very little bias. Quite inefficient however. Possible implemntation would focus on creating walls and be much faster (since the entire boundary is a wall) so the loop only has to find the boundary rather than the single visited cell as in the current implementation. Would require a rework of existing code however.</p>
</details>

<details><summary>Aldous Broder Algorithm</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Maze-Gen/blob/main/imgs/colourful_AldousBroder.png" />
  </p>
  <p>Random walk. Very inefficient but creates mazes with little bias.</p>
</details>

<details><summary>Sidewinder Algorithm</summary>
  <p align="center">
    <img src="https://github.com/JPDye/Maze-Gen/blob/main/imgs/colourful_sidewinder.png" />
  </p>
  <p>Randomly descide whether to carve east or north. If east is chosen add the cell to the current "run". If north is chosen pick a cell in the run and move north from that cell (if possible) and end the run. Repeat until all cells are visited. Creates mazes with an empty passage at the top and a bias for passages running to the north east.</p>
</details>

<details><summary>Binary Tree Algorithm</summary> 
  <p align="center">
    <img src="https://github.com/JPDye/Maze-Gen/blob/main/imgs/colourful_binary_tree.png" />
  </p>

  <p>Randomly decide to carve either north or east. If north isn't possible, carve east. If east isn't possible, carve north. If neither can be done, do nothing. This algorithm creates mazes with an empty passage at the north and east of the maze with a strong bias for passages running to the north east.</p>
</details>

## ToDo
* Hexagon mazes.
