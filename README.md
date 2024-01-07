# Ray Tracing in One Weekend (in Rust)

![Final scene rendered using the raytracer with threads](img/final_scene.png "Final Render")

An implementation of the ray tracer from [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley in [Rust](https://www.rust-lang.org/).
While the file and class structure is largely the same, I made a few changes to translate the original C++ code into
more idiomatic Rust. The translation was otherwise fairly straightforward.

For example,

* Returning an `Option<T>` instead of `-1` for functions like `hit` when we don't hit an object and don't have a value to return. This also gets rid of having to pass a reference to the output variable and assigning it within a function.
* Replacing `std::shared_ptr` with `std::rc::Rc` (thank you to pie_flavor#7868 from [The Rust Programming Language Discord](https://discord.gg/rust-lang) for helping me understand lifetimes better 😊)

## Usage

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release > output.ppm
```

## Future Work

* [x] Parallelism (cuts down the time to generate the final scene from 1:03:55.63s with 1 thread, to 25:14.6s with 6 threads; a 61% improvement)
* [ ] Triangles (implement model I/O)
* [ ] Lights (will make everything look prettier; use shadow rays or bias rays towards lights with downweighting)
* [ ] Surface Textures (will make everything look prettier)
* [ ] Solid Textures (generative textures, Perlin noise)
* [ ] Volumes and Media (make volumes have hittable surfaces with probabilistic intersections based on density)
