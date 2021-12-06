# Day 5 of AOC - Hydrothermal Venture

We're being asked to process a set of vector coordinates in the form 'x1,y1 -> x2,y2' and figure out the number of points 
that have intersecting lines in order to avoid some imaginary hydrothermal vents.

The first part says to only consider horizontal and vertical lines ie: lines where either x1 = x2 or y1 = y2. so we should expect Part 2 to require us to consider diagonals as well. Looking through some of the data, I suspect we only have 45 degree angles to worry about though, which makes that a lot easier.


## Process:
1. Data wrangling: We will want to parse the inputs into two x,y end-points
2. Need to get all points between two end-points (using a cool Bresenham library to save time)
3. Need to be able to determine if line is diagonal 
4. Figure out # that each time a Point appears
5. Get count of Points that appeared more than once

# Learning Rust

My first thought was to use some Class to keep an x,y coordinate board in memory and have it process some Points but I dont think I'll go for it. For one, the board size is not given as a fact and that is a show stopper. 

Being that this is a learning exercise, I decided to still go for some OOP and implemented a Line class instead. The idea is that a Line is instantiated directly from the input string and would encapsulate the state and business logic required to figure out how to get all the points between the start and end of the line.

I then inserted each point along the lines into a HashMap and used the return value of the 'insert' function to determine if each point is unique or duplicate.