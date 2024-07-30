# Ray Trasing by Rust

on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

<!-- memo -->
## ss\_2

I implemented program which generat \*\.ppm image file.

result([src/ss\_2/ppm\_img.rs](src/ss_2/ppm_img.rs))

<img src=fig/first_ppm_image.svg>

## ss\_3

I implemented 3-dimentional vector library and utility.

result([src/ss\_3/ppm\_img.rs](src/ss_3/ppm_img.rs))

<img src=fig/first_ppm_image.svg>

## ss\_4

I implemented struct Ray.

$$
\boldsymbol{P}(t) = \boldsymbol{A} + t\boldsymbol{b}
$$

result([src/ss\_4/scene\_rays.rs](src/ss_4/scene_rays.rs))

<img src=fig/scene_rays.ppm.svg>

## ss\_5

### shpere:

for point $\boldsymbol{P} = (x,y,z), \boldsymbol{C} = (C_x, C_y,C_z)$

$$
|\boldsymbol{P} - \boldsymbol{C}| = r
$$

If ray $\boldsymbol{P}(t) = \boldsymbol{Q} + t\boldsymbol{b}$ hit sphere, $\boldsymbol{P}(t)$ satisfies below.

$$
\begin{aligned}
    &|\boldsymbol{P}(t) - \boldsymbol{C}| = r\\
    &|\boldsymbol{Q} + t\boldsymbol{d} - \boldsymbol{C}| = r
\end{aligned}
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
    &b = -2\boldsymbol{d}\cdot(\boldsymbol{C} - \boldsymbol{Q})\\
    &c =  (\boldsymbol{C} - \boldsymbol{Q})\cdot(\boldsymbol{C} - \boldsymbol{Q}) - r^2
\end{cases}
$$

and discreminant of quadratic equation $D$ is devined by below

$$
D = b^2 - 4ac
$$

root is

$$
t = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$

more simple

$$
t = \frac{h \pm \sqrt{h^2 - ac}}{a} \quad \left(h = \frac{-b}{2}\right)
$$

result([src/ss\_5/add\_sphere.rs](src/ss_5/add_sphere.rs))

<img src=fig/add_sphere.ppm.svg>


## ss\_6

- use:
    - std::rc::Rc\<T\>
    - std::cell::RefCell\<T\>
    - trait
    - generic struct
    - trait bounder


result\_1([src/ss\_6/sphere\_surface.rs](src/ss_6/sphere_surface.rs))

<img src=fig/sphere_surface.ppm.svg>


result\_2([src/ss\_6/sphere\_surface\_2.rs](src/ss_6/sphere_surface_2.rs))

<img src=fig/sphere_surface_2.ppm.svg>

## ss\_7

implemented struct Camera and refacterd main()

## ss\_8

antialiasing

result([src/ss\_8/antialiasing.rs](src/ss_8/antialiasing.rs))

<img src=fig/antialiasing.ppm.svg>

## ss\_9

diffuse materials

result\_1([src/ss\_9/diffuse_sphere.rs](src/ss_9/diffuse_sphere.rs))

<img src=fig/diffuse_sphere.ppm.svg>
