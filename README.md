# Multithreaded Raytracer in Rust

A **simple multithreaded raytracer** implemented in Rust, featuring:

- **Metals**
- **Dialectrics**
- **Textures**
- **Anti-Aliasing**
- **Blur**
- **Perlin Noise Generation**
- **And much more!**

---

## 🚀 Running the Program

### Clone the Repository
To get started, either **clone the repository** or **download the release** for your platform.

### Basic Commands
#### Run without Disk Sampling:
```bash
cargo run --release <scene-number>
raytracer <scene-number>
```

#### Run with Disk Sampling:
```bash
cargo run --release <scene-number> <disk-sampling>
raytracer <scene-number> <disk-sampling>
```

#### List Available Scenes:
```
cargo run --release list
raytracer list
```
---
### 🛠️ Things to Implement in the Future

- **Volume Effects**
- **BVH (Bounding Volume Hierarchy)**
- **Instances**
- **GPU Support**

---
### 📚 Resources
- **Raytracing In A Weekend Series:** https://raytracing.github.io
- **The Ray Tracer Challenge:** http://raytracerchallenge.com
 
