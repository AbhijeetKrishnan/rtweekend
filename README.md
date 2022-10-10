# Ray Tracing in One Weekend (in Rust)

An implementation of the ray tracer from [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend) by Peter Shirley in [Rust](https://www.rust-lang.org/).
While the file and class structure is largely the same, I made a few changes to translate the original C++ code into
more idiomatic Rust. The translation was otherwise fairly straightforward.

* Returning an `Option<T>` instead of `-1` for cases like `hit` when we don't hit an object and don't have a value to return. This also gets rid of having to pass a reference to the output variable and assigning it within a function.
* Replacing `std::shared_ptr` with `std::rc::Rc` (thank you to pie_flavor#7868 from [The Rust Programming Language Discord](https://discord.gg/rust-lang) for helping me understand lifetimes better 😊)

Here are some images that were rendered due to bugs in my code.



## Future Work

* [ ] Parallelism (rendering the final scene takes a *huge* amount of time)
* [ ] Triangles (implement model I/O)
* [ ] Lights (will make everything look prettier; use shadow rays or bias rays towards lights with downweighting)
* [ ] Surface Textures (will make everything look prettier)
* [ ] Solid Textures (generative textures)
* [ ] Volumes and Media (make volumes have hittable surfaces with probabilistic intersections based on density)