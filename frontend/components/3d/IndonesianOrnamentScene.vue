<script setup lang="ts">
import * as THREE from 'three'

const containerRef = ref<HTMLDivElement | null>(null)

let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let animationFrameId: number

// 3D Objects
const ornamentGroup = new THREE.Group()
let directionalLight: THREE.DirectionalLight
let pointLight: THREE.PointLight

// Mouse tracking with smooth damping
const mouse = { x: 0, y: 0, targetX: 0, targetY: 0 }

const initThree = () => {
  if (!containerRef.value) return

  const width = containerRef.value.clientWidth
  const height = containerRef.value.clientHeight

  // 1. Scene
  scene = new THREE.Scene()

  // 2. Camera
  camera = new THREE.PerspectiveCamera(40, width / height, 0.1, 1000)
  camera.position.set(0, 0, 8.5)

  // 3. WebGL Renderer with High Precision & Soft Shadows
  renderer = new THREE.WebGLRenderer({
    alpha: true,
    antialias: true,
    powerPreference: 'high-performance'
  })
  renderer.setSize(width, height)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap

  containerRef.value.appendChild(renderer.domElement)

  // 4. Studio Gallery Lighting
  // Soft ambient
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)

  // Grazing Directional Key Light (Warm Gallery Spot)
  directionalLight = new THREE.DirectionalLight(0xf5eedc, 3.2)
  directionalLight.position.set(5, 7, 6)
  directionalLight.castShadow = true
  directionalLight.shadow.mapSize.width = 2048
  directionalLight.shadow.mapSize.height = 2048
  directionalLight.shadow.bias = -0.0001
  scene.add(directionalLight)

  // Muted Emerald/Bronze Rim Light
  const rimLight = new THREE.DirectionalLight(0x34d399, 1.4)
  rimLight.position.set(-6, -5, -4)
  scene.add(rimLight)

  // Point Light for dramatic specular highlights on geometric facets
  pointLight = new THREE.PointLight(0xd4af37, 1.8, 12)
  pointLight.position.set(0, 2, 4)
  scene.add(pointLight)

  // 5. Construct Contemporary Indonesian Geometric Sculpture (Parametric Woven Relief)
  buildAbstractIndonesianSculpture()

  scene.add(ornamentGroup)

  // Initial sculpture angle
  ornamentGroup.rotation.x = 0.2
  ornamentGroup.rotation.y = -0.35

  animate()
}

// Builds an abstract contemporary relief sculpture inspired by Indonesian geometric linework / woven structures
const buildAbstractIndonesianSculpture = () => {
  // Material 1: Matte Charcoal Architectural Ceramic with subtle metallic sheen
  const ceramicMaterial = new THREE.MeshStandardMaterial({
    color: 0x181e28,
    roughness: 0.35,
    metalness: 0.25,
    flatShading: true
  })

  // Material 2: Muted Bronze / Warm Titanium Accent
  const bronzeMaterial = new THREE.MeshStandardMaterial({
    color: 0x8a7a60,
    roughness: 0.3,
    metalness: 0.7,
    flatShading: true
  })

  // Material 3: Subtle Emerald Tinted Core
  const emeraldMaterial = new THREE.MeshStandardMaterial({
    color: 0x064e3b,
    roughness: 0.4,
    metalness: 0.5,
    flatShading: true
  })

  // Central Octagonal / Diamond Layered Medallion (inspired by Ceplok/Kawung geometry)
  const layerCount = 6
  for (let l = 0; l < layerCount; l++) {
    const radius = 1.6 - l * 0.22
    const segments = 8
    const shape = new THREE.Shape()

    for (let i = 0; i <= segments; i++) {
      const theta = (i / segments) * Math.PI * 2
      const r = radius * (1 + 0.15 * Math.sin(theta * 4)) // Subtle 4-fold floral/geometric perturbation
      const x = Math.cos(theta) * r
      const y = Math.sin(theta) * r
      if (i === 0) shape.moveTo(x, y)
      else shape.lineTo(x, y)
    }

    const extrudeSettings = {
      steps: 1,
      depth: 0.08,
      bevelEnabled: true,
      bevelThickness: 0.04,
      bevelSize: 0.03,
      bevelSegments: 2
    }

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings)
    const mat = l % 2 === 0 ? ceramicMaterial : bronzeMaterial
    const mesh = new THREE.Mesh(geometry, mat)

    mesh.position.z = (l - layerCount / 2) * 0.12
    mesh.rotation.z = l * (Math.PI / 8) // Cascading interlocking rotation
    mesh.castShadow = true
    mesh.receiveShadow = true

    ornamentGroup.add(mesh)
  }

  // Interlocking Geometric Orbit Ribbons (Parametric woven rings)
  const ringCount = 3
  for (let r = 0; r < ringCount; r++) {
    const torusGeo = new THREE.TorusGeometry(2.3 + r * 0.35, 0.022, 16, 64)
    const torusMesh = new THREE.Mesh(torusGeo, bronzeMaterial)
    torusMesh.rotation.x = Math.PI / 3 + r * 0.4
    torusMesh.rotation.y = r * (Math.PI / 4)
    torusMesh.castShadow = true
    ornamentGroup.add(torusMesh)
  }

  // Floating Micro Prisms (Subtle rhythmic floating points)
  const prismGeo = new THREE.OctahedronGeometry(0.09, 0)
  for (let p = 0; p < 16; p++) {
    const pMesh = new THREE.Mesh(prismGeo, emeraldMaterial)
    const angle = (p / 16) * Math.PI * 2
    const dist = 2.8 + Math.sin(p * 3) * 0.4
    pMesh.position.set(
      Math.cos(angle) * dist,
      Math.sin(angle) * dist * 0.8,
      (Math.sin(p * 2) - 0.5) * 0.8
    )
    pMesh.rotation.set(p, p * 0.5, 0)
    pMesh.castShadow = true
    ornamentGroup.add(pMesh)
  }
}

const animate = () => {
  animationFrameId = requestAnimationFrame(animate)

  // Smooth lerp mouse coordinates
  mouse.x += (mouse.targetX - mouse.x) * 0.04
  mouse.y += (mouse.targetY - mouse.y) * 0.04

  const time = performance.now() * 0.0006

  // Slow, cinematic ambient drift
  const driftY = Math.sin(time * 0.8) * 0.06
  const driftRot = Math.cos(time * 0.5) * 0.04

  // Interactive rotation with cursor influence
  ornamentGroup.rotation.y = -0.35 + mouse.x * 0.4 + driftRot
  ornamentGroup.rotation.x = 0.2 - mouse.y * 0.3 + Math.sin(time * 0.6) * 0.03
  ornamentGroup.position.y = driftY

  // Move dynamic light subtly with cursor to reveal embossed surface facets
  directionalLight.position.x = 5 + mouse.x * 3
  directionalLight.position.y = 7 + mouse.y * 3
  pointLight.position.x = mouse.x * 2
  pointLight.position.y = 2 + mouse.y * 2

  renderer.render(scene, camera)
}

const handleMouseMove = (e: MouseEvent) => {
  if (!containerRef.value) return
  const rect = containerRef.value.getBoundingClientRect()
  mouse.targetX = ((e.clientX - rect.left) / rect.width) * 2 - 1
  mouse.targetY = -(((e.clientY - rect.top) / rect.height) * 2 - 1)
}

const handleResize = () => {
  if (!containerRef.value || !renderer || !camera) return
  const width = containerRef.value.clientWidth
  const height = containerRef.value.clientHeight
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

onMounted(() => {
  initThree()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  if (animationFrameId) cancelAnimationFrame(animationFrameId)
  if (renderer && renderer.domElement) {
    renderer.dispose()
  }
})
</script>

<template>
  <div
    ref="containerRef"
    class="w-full h-full cursor-crosshair select-none relative"
    @mousemove="handleMouseMove"
    @mouseleave="() => { mouse.targetX = 0; mouse.targetY = 0; }"
  >
    <!-- Subtle gradient vignetting for gallery atmosphere -->
    <div class="absolute inset-0 pointer-events-none bg-radial from-transparent via-transparent to-canvas opacity-80" />
  </div>
</template>
