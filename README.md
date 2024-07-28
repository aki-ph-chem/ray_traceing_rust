# Ray Trasing by Rust

<!-- memo -->
## ss\_2

I implemented program which generat \*\.ppm image file.

## ss\_3

I implemented 3-dimentional vector library and utility.

## ss\_4

I implemented struct Ray.

$$
\boldsymbol{P}(t) = \boldsymbol{A} + t\boldsymbol{b}
$$

## ss\_5

### shpere:

for point $\boldsymbol{P} = (x,y,z), \boldsymbol{C} = (C_x, C_y,C_z)$

$$
|\boldsymbol{P} - \boldsymbol{C}| = r
$$

If ray $\boldsymbol{P}(t) = \boldsymbol{Q} + t\boldsymbol{b}$ hit sphere, $\boldsymbol{P}(t)$ satisfies below.

$$
|\boldsymbol{P}(t) - \boldsymbol{C}| = r\\
|\boldsymbol{Q} + t\boldsymbol{d} - \boldsymbol{C}| = r
$$

square both sides

$$
\begin{aligned}
&|\boldsymbol{Q} + t\boldsymbol{d} - \boldsymbol{C}|^2 = r^2\\
&(\boldsymbol{d}\cdot\boldsymbol{d}) t^2 - 2\{\boldsymbol{d}\cdot(\boldsymbol{C} - \boldsymbol{Q})\}t + (\boldsymbol{C} - \boldsymbol{Q})\cdot(\boldsymbol{C} - \boldsymbol{Q}) = r^2\\
&(\boldsymbol{d}\cdot\boldsymbol{d}) t^2 - 2\{\boldsymbol{d}\cdot(\boldsymbol{C} - \boldsymbol{Q})\}t + (\boldsymbol{C} - \boldsymbol{Q})\cdot(\boldsymbol{C} - \boldsymbol{Q}) - r^2 = 0
\end{aligned}
$$

quadratic equation

$$
at^2 + bt + c = 0
$$

coefficient $a,b,c$ is defined by below

$$
\begin{cases}
    &a = \boldsymbol{d}\cdot\boldsymbol{d}\\
    &b = -2\boldsymbol{d}\cdot(\boldsymbol{C}\\
    &c =  (\boldsymbol{C} - \boldsymbol{Q})\cdot(\boldsymbol{C} - \boldsymbol{Q}) - r^2
\end{cases}
$$

and discreminant of quadratic equation $D$ is devined by below

$$
D = b^2 - 4ac
$$


## ss\_6
