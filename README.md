# 📌 3D Map Engine — Standalone Version

## 🚀 Project Summary

**3D Map Engine** est un moteur 3D haute performance et léger pour **assembler, traiter et visualiser des cartes géographiques à partir de données existantes** — sans dépendre initialement de flux de drones.

Cible : professionnels disposant de données topographiques (images aériennes, LIDAR, DEM, GPS/IMU) et besoin d’un traitement local rapide, sans cloud lourd.

---

## ✅ Key Features

- **Multi-format input**  
  Supporte : images aériennes, nuages de points LIDAR, modèles numériques d’élévation (DEM), métadonnées GPS/IMU.

- **Structure-from-Motion (SfM)**  
  Alignement automatique des images et reconstruction 3D cohérente.

- **Data Cleaning & Correction**  
  Filtrage des outliers, correction colorimétrique, optimisation de la géométrie.

- **Level of Detail (LOD)**  
  Génération automatique de multiples résolutions pour un rendu fluide.

- **Real-Time 3D Viewer**  
  Visualisation interactive, navigation intuitive, outils de mesure, superpositions et annotations.

- **Export**  
  Formats standard : GeoTIFF, OBJ, STL, LAS.

- **Modular Rust Architecture**  
  Séparation claire : gestion I/O, moteur de calcul, renderer, orchestrateur.

- **Optional Analysis Tools**  
  Classification basique, détection de changements entre relevés.

- **Future Option: Ortho Map Generation**  
  À terme, prise en charge de l’import d’**images brutes** (non alignées) pour générer automatiquement une **orthomap** haute résolution.

---

## ⚙️ Project Structure

```plaintext
3d-map-engine/
├── io_manager/         # Ingestion multi-format, parsing des métadonnées
├── core/               # Moteur de SfM, pipeline de traitement, génération LOD
├── renderer/           # Moteur de rendu 3D interactif
├── orchestrator/       # Orchestration des modules, gestion des workflows
├── cli/                # Interface en ligne de commande
├── examples/           # Jeux de données de test
└── docs/               # Documentation technique et usage
```

---

## 🗂️ Development Roadmap

### 1️⃣ **Input & I/O**
- [ ] Lecture des formats : GeoTIFF, LAS, DEM.
- [ ] Parser GPS/IMU (Exif, CSV, JSON).
- [ ] [Future] Support d’import d’images brutes pour génération d’orthomap.

### 2️⃣ **SfM Pipeline**
- [ ] Détection et appariement de features.
- [ ] Estimation des poses caméra.
- [ ] Génération du nuage de points sparse & dense.

### 3️⃣ **Post-Processing**
- [ ] Filtrage des outliers.
- [ ] Correction colorimétrique.
- [ ] Mesh generation + texture.

### 4️⃣ **LOD & Tiling**
- [ ] Algorithme pour multi-résolutions.
- [ ] Génération de tuiles.

### 5️⃣ **Renderer**
- [ ] Visualiseur 3D temps réel en Rust (OpenGL / Vulkan).
- [ ] Navigation fluide, mesure, overlays.

### 6️⃣ **Export**
- [ ] Export GeoTIFF, OBJ, STL, LAS.
- [ ] [Future] Export orthomap géoréférencée.

### 7️⃣ **Orchestrator & CLI**
- [ ] Orchestration des modules.
- [ ] Interface CLI ergonomique.

### 8️⃣ **Optional Analysis**
- [ ] Classification basique.
- [ ] Détection de changements entre relevés.

---

## 📖 How to Build

```bash
# Clone

# Build (Rust)
cargo build --release

# Run example pipeline
cargo run -- --input ./examples/dataset1 --output ./output/
```

---

## 📌 Long-Term Vision

- Intégrer un flux en direct de drones plus tard.
- Backend distribué et scalable.
- Compatible edge devices, optimisé pour performance maximale en Rust.
- Ajout de la génération automatique d’orthomap à partir d’images brutes.
- peut etre un moteur de rendu maison

---
