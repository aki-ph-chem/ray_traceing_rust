# Ray Trasing by Rust

on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

<!-- memo -->
## ss\_2

to save result as file

```Rust
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 2 {
        panic!("Error: invalid args");
    }
    let file_name = argv[1].clone();


    /*
       some code
     */

    std::writeln!(&mut file, some_string)?;

    Ok(())
}
```

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

- ray tracer
    1. clculate the ray from the "eye" through the pixel
    2. determine which object the ray intersects
    3. compute a color for the closest intersection point

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
    - Rc\<RefCell\<T\>\>
    - trait
    - generic struct
    - trait bounder

in C++ define `abstract class Hittable` and inherit it from some object.

in Rust define `trait Hittable` and implemented it to some struct.

```Rust
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
```

in this code I implemented it for `struct Sphere`
```Rust
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        /*
        some code
        */
    }
}
```

in C++(this text) use std::vector\<std::shared\_ptr\<T\>\>

in Rust use Vec\<Rc\<RefCell\<T\>\>\>

```Rust
pub struct HittableList<T> {
    pub objects: Vec<Rc<RefCell<T>>>,
}
```

and implemented `trait Hittable`

```Rust
impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        /*
        some code
        */
    }
}
```

and define `ray_color()`

```Rust
fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
}
```

result\_1([src/ss\_6/sphere\_surface.rs](src/ss_6/sphere_surface.rs))

<img src=fig/sphere_surface.ppm.svg>


result\_2([src/ss\_6/sphere\_surface\_2.rs](src/ss_6/sphere_surface_2.rs))

<img src=fig/sphere_surface_2.ppm.svg>

## ss\_7

implemented struct Camera and refacterd main()

## ss\_8

antialiasing

for random vector

```Rust
use rand;
use rand::Rng;

pub struct Random {
    rng: rand::rngs::ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn random_f64(&mut self) -> f64 {
        self.rng.gen_range(0.0..1.0)
    }

    pub fn random_f64_range(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min..max)
    }
}
```

result([src/ss\_8/antialiasing.rs](src/ss_8/antialiasing.rs))

<img src=fig/antialiasing.ppm.svg>

## ss\_9

diffuse materials

result\_1([src/ss\_9/diffuse\_sphere.rs](src/ss_9/diffuse_sphere.rs))

<img src=fig/diffuse_sphere.ppm.svg>

lambertian

result\_2([src/ss\_9/lambertian.rs](src/ss_9/lambertian.rs))

<img src=fig/lambertian.ppm.svg>

gamma correction

gamma correction is defined by

$$
y = Ax^{\gamma}
$$

```Rust
fn linear_to_gamma(linear_component: f64, gamma: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.powf(gamma);
    }

    0.0
}

pub fn write_color_gamma<T: std::io::Write>(
    gamma: f64,
    out: &mut T,
    pixel_color: &Color,
) -> Result<(), Box<dyn std::error::Error>> {
    let (r, g, b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());
    let (r, g, b) = (
        linear_to_gamma(r, gamma),
        linear_to_gamma(g, gamma),
        linear_to_gamma(b, gamma),
    );

    let intensity = Interval::new_by_value(0.00, 0.999);
    let (r_byte, g_byte, b_byte) = (
        (256.000 * intensity.clamp(r)) as i32,
        (256.000 * intensity.clamp(g)) as i32,
        (256.000 * intensity.clamp(b)) as i32,
    );

    std::writeln!(out, "{r_byte} {g_byte} {b_byte}")?;
    Ok(())
}
```

result\_2([src/ss\_9/gamma\_correction.rs](src/ss_9/gamma_correction.rs))

gamma = 1.5

<img src=fig/gamma_1.5.ppm.svg>

gamma = 1.0

<img src=fig/gamma_1.0.ppm.svg>

gamma = 0.5

<img src=fig/gamma_0.5.ppm.svg>

gamma = 0.25

<img src=fig/gamma_0.25.ppm.svg>

## ss\_10

add material

define `trait Material` and `struct HitRecordMat` as below to avoid circular reference of `trait Material`.


```Rust
pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecordMat,
        _attennuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
```

```Rust
#[derive(Clone)]
pub struct HitRecordMat {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<RefCell<dyn Material>>>,
    pub t: f64,
    pub front_face: bool,
}
```

point:

not

```Rust
pub mat: Rc<RefCell<T>>,
```
under the trait bunder `T: Material<T>`

but in

```Rust
pub mat: Option<Rc<RefCell<dyn Material>>>,
```

result\_1([src/ss\_10/metal\_sphere.rs](src/ss_10/metal_sphere.rs))

<img src=fig/sphere_metal.ppm.svg>

result\_2([src/ss\_10/metal\_sphere\_fuzz.rs](src/ss_10/metal_sphere_fuzz.rs))

<img src=fig/sphere_metal_fuzz.ppm.svg>

## ss\_11

Snell's law

$$
\begin{aligned}
    &\eta\sin \theta = \eta^{\prime} \sin \theta^{\prime}\\
    &\sin \theta^{\prime} = \frac{\eta}{\eta^{\prime}} \sin \theta
\end{aligned}
$$

the refracted ray is splited $\boldsymbol{R}^{\prime}$ into two parts.
papendicular ot $\boldsymbol{n}^{\prime}$ and parallel to $\boldsymbol{n}^{\prime}$

$$
\boldsymbol{R}^{\prime} = \boldsymbol{R}^{\prime}_{per} + \boldsymbol{R}^{\prime}_{para}
$$

$\boldsymbol{R}^{\prime}_{per}$ mean refract <b/>pependicular</b>

$\boldsymbol{R}^{\prime}_{para}$ mean refract <b/>parallel</b>

$$
\begin{cases}
    &\boldsymbol{R}^{\prime}_{per} = \frac{\eta}{\eta^{\prime}}
    \left\{
        \boldsymbol{R} + (-\boldsymbol{R} \cdot \boldsymbol{n})\boldsymbol{n}
    \right\}\\
    &\boldsymbol{R}^{\prime}_{par} = -\left(\sqrt{1 - |\boldsymbol{R}^{\prime}_{per}|^2}\right) \boldsymbol{n}
\end{cases}
$$

result\_1([src/ss\_11/grass\_sphere.rs](src/ss_11/grass_sphere.rs))

<img src=fig/glass_sphere_1.ppm.svg>

result\_2([src/ss\_11/grass\_sphere\_2.rs](src/ss_11/grass_sphere_2.rs))

<img src=fig/glass_sphere_2.ppm.svg>

result\_3([src/ss\_11/grass\_sphere\_3.rs](src/ss_11/grass_sphere_3.rs))

<img src=fig/glass_sphere_3.ppm.svg>
