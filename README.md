# Billiard multiverse simulation

I was inspired by a scientific publication[[1]](#ref-1) which demonstrated that a discrete (quantized) space-time inevitably leads to multiple possible paths given any initial conditions.

I wanted to visualize this fact by trying to reproduce the computations this paper is about, so I created this small physics simulation. As I recently learned the Rust language, it was a good opportunity to practice it.

The simulation consists of hard-sphere elastic collisions computation in 2D frictionless billiards. 2 billiards are initialized with the same initial conditions given a defined Epsilon precision. The initial position of ball n in the first billiard, is the position of ball n in the second billiard + a delta of magnitude Epsilon. Initial velocities are exactly the same.

The [Bevy](https://bevyengine.org/) ECS game engine is used to visualize the simulation. The balls from the first billiard are red, and the ones from the second billiard are blue. When the stories of the billiard start to differ, red and blue balls stop to be supperposed.

## References

- [1] <a name="ref-1"></a>_Guillemant, P., Medale, M., & Abid, C._ (2018). **A discrete classical space–time could require 6 extra-dimensions.** In Annals of Physics (Vol. 388, pp. 428–442). Elsevier BV. [![DOI:10.1007/978-3-031-21438-7_60](https://zenodo.org/badge/DOI/10.1007/978-3-319-76207-4_15.svg)](https://doi.org/10.1016/j.aop.2017.11.023)
