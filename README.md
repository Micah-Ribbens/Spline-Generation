# Summary
This project is the Spline Generation code for the 2024 Utah Lunabotics. High level, this is the code that generates 
the spline using control points. The spline will pass through all the points using C<sup>2</sup> continuity. This 
means the spline is continuous up until the second derivative and is therefore a cubic spline. This project has the 
spline generation code with some pseudocode for running it on the robot (filler motor and electrical APIs). 
This project will also have a simulation of the robot following the spline path.

# How it works
The main API call that the user of the application will use the spline generation function. The spline generation function 
will have these parameters:
1. Control Points &rarr; The points that the spline will pass through. Each control point has an x and a y coordinate.
2. Way Points &rarr; The commands that will be run at specific points along the path. Each way point will have a t-value 
and a command. That command can be any function that only has one parameter: the current t-value of the robot.
3. Initial Velocity &rarr; The initial x and y vector of the robot at the start of the spline.
4. Final Velocity &rarr; The final x and y vector of the robot at the end of the spline.

Then using that information and gaussian elimination, the values of the polynomial are solved. Once the path is defined, 
I will be using some conversion math to turn the velocity vector into a driver motor power and a rotation motor power. 
The powers will be bounded by between a min and max value for each. This guarantees that the motors will not be overrun 
causing unnecessary wear and tear. It also guarantees the robot will not stop (even a low motor power can be not enough 
to overcome friction).

# Let's walkthrough a real word example
TODO add a real world example