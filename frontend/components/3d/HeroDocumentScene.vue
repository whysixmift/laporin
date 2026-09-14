<script setup lang="ts">
import * as THREE from 'three'

const containerRef = ref<HTMLDivElement | null>(null)
const isHovered = ref(false)
const isFanned = ref(false)
const activeTab = ref<'assembled' | 'fanned' | 'notes'>('assembled')

let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let animationFrameId: number

// 3D Objects
const pagesGroup = new THREE.Group()
const notesGroup = new THREE.Group()
const mainRig = new THREE.Group()

// Mouse tracking with smooth lerp
const mouse = { x: 0, y: 0, targetX: 0, targetY: 0 }

const initThree = () => {
  if (!containerRef.value) return

  const width = containerRef.value.clientWidth
  const height = containerRef.value.clientHeight

  // 1. Scene
  scene = new THREE.Scene()

  // 2. Camera
  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000)
  camera.position.set(0, 0, 7.5)

  // 3. Renderer with antialiasing and high pixel ratio
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

  // 4. Studio Lighting (Warm Key + Cool Rim + Ambient)
  const ambientLight = new THREE.AmbientLight(0xffffff, 1.2)
  scene.add(ambientLight)

  const keyLight = new THREE.DirectionalLight(0xfff8f0, 2.4)
  keyLight.position.set(4, 6, 5)
  keyLight.castShadow = true
  keyLight.shadow.mapSize.width = 1024
  keyLight.shadow.mapSize.height = 1024
  scene.add(keyLight)

  const rimLight = new THREE.DirectionalLight(0x34d399, 1.8) // Emerald rim
  rimLight.position.set(-6, -4, -3)
  scene.add(rimLight)

  const topFillLight = new THREE.PointLight(0xe0e7ff, 1.0, 15)
  topFillLight.position.set(0, 4, 3)
  scene.add(topFillLight)

  // 5. Build Physical A4 Pages (Layered Stack)
  buildDocumentStack()

  // 6. Build Floating Raw Notes (Chaotic Input Stage)
  buildFloatingNotes()

  mainRig.add(pagesGroup)
  mainRig.add(notesGroup)
  scene.add(mainRig)

  // Initial tilt
  mainRig.rotation.x = 0.15
  mainRig.rotation.y = -0.25

  animate()
}

// Canvas Texture Generator for realistic paper with printed Indonesian report typography
const createPageTexture = (title: string, sub: string, bab: string, isCover = false) => {
  const canvas = document.createElement('canvas')
  canvas.width = 1024
  canvas.height = 1448 // A4 ratio
  const ctx = canvas.getContext('2d')
  if (!ctx) return new THREE.CanvasTexture(canvas)

  // Paper base color (Warm off-white academic paper)
  ctx.fillStyle = isCover ? '#12161f' : '#f8fafc'
  ctx.fillRect(0, 0, canvas.width, canvas.height)

  // Subtle paper grain/grid margin line
  ctx.strokeStyle = isCover ? '#2d3748' : '#e2e8f0'
  ctx.lineWidth = 2
  ctx.strokeRect(80, 80, canvas.width - 160, canvas.height - 160)

  if (isCover) {
    // Cover Page Styling
    ctx.fillStyle = '#34d399' // emerald
    ctx.font = 'bold 28px monospace'
    ctx.textAlign = 'center'
    ctx.fillText('LAPORAN PRAKTIK KERJA LAPANGAN (PKL)', canvas.width / 2, 220)

    ctx.fillStyle = '#ffffff'
    ctx.font = 'bold 44px sans-serif'
    ctx.fillText(title, canvas.width / 2, 360)

    ctx.fillStyle = '#94a3b8'
    ctx.font = '32px sans-serif'
    ctx.fillText(sub, canvas.width / 2, 430)

    // Seal / Emblem placeholder box
    ctx.strokeStyle = '#34d399'
    ctx.strokeRect(canvas.width / 2 - 80, 560, 160, 160)
    ctx.fillStyle = '#34d399'
    ctx.font = 'bold 24px monospace'
    ctx.fillText('SMK / VOKASI', canvas.width / 2, 650)

    // Author metadata
    ctx.fillStyle = '#cbd5e1'
    ctx.font = '26px sans-serif'
    ctx.fillText('Disusun Oleh:', canvas.width / 2, 880)
    ctx.fillStyle = '#ffffff'
    ctx.font = 'bold 30px sans-serif'
    ctx.fillText('Julian Mifta Yama Fauzan', canvas.width / 2, 930)
    ctx.fillStyle = '#94a3b8'
    ctx.font = '24px monospace'
    ctx.fillText('NISN: 0072918239', canvas.width / 2, 975)

    ctx.fillStyle = '#64748b'
    ctx.font = '24px sans-serif'
    ctx.fillText('KEMENTERIAN PENDIDIKAN DAN KEBUDAYAAN', canvas.width / 2, 1260)
    ctx.fillText('TAHUN AJARAN 2025/2026', canvas.width / 2, 1300)
  } else {
    // Academic Page Styling
    ctx.fillStyle = '#0f172a'
    ctx.font = 'bold 36px serif'
    ctx.textAlign = 'center'
    ctx.fillText(bab, canvas.width / 2, 180)
    ctx.font = 'bold 28px sans-serif'
    ctx.fillText(title, canvas.width / 2, 230)

    // Horizontal line
    ctx.beginPath()
    ctx.moveTo(120, 260)
    ctx.lineTo(canvas.width - 120, 260)
    ctx.strokeStyle = '#cbd5e1'
    ctx.stroke()

    // Paragraph Lines simulation
    ctx.fillStyle = '#334155'
    ctx.font = '22px serif'
    ctx.textAlign = 'left'

    const lines = [
      '1.1 Latar Belakang Praktik Kerja Lapangan',
      'Praktik Kerja Lapangan (PKL) merupakan wahana strategis dalam menjembatani',
      'kesiapan kompetensi peserta didik dengan kebutuhan riil Dunia Usaha dan',
      'Dunia Industri (DU/DI). Melalui program ini, praktikan memperoleh pengalaman',
      'nyata mengenai etos kerja, disiplin industri, dan pemecahan masalah teknis.',
      '',
      '1.2 Maksud dan Tujuan Pelaksanaan',
      'Adapun tujuan yang hendak dicapai dalam pelaksanaan magang ini meliputi:',
      '1. Menerapkan keahlian rekayasa perangkat lunak pada arsitektur produksi.',
      '2. Memahami alur kerja kolaboratif pada divisi teknologi dan sistem informasi.',
      '3. Memenuhi standar kelulusan akademik vokasi tahun ajaran 2025/2026.'
    ]

    let y = 330
    for (const l of lines) {
      if (l.startsWith('1.')) {
        ctx.font = 'bold 24px sans-serif'
        ctx.fillStyle = '#0f172a'
      } else {
        ctx.font = '22px serif'
        ctx.fillStyle = '#334155'
      }
      ctx.fillText(l, 120, y)
      y += 42
    }

    // Page number
    ctx.fillStyle = '#64748b'
    ctx.font = '20px monospace'
    ctx.textAlign = 'center'
    ctx.fillText('Hal. ' + sub, canvas.width / 2, 1360)
  }

  const texture = new THREE.CanvasTexture(canvas)
  texture.anisotropy = 8
  return texture
}

const pageMeshes: THREE.Mesh[] = []

const buildDocumentStack = () => {
  // A4 Geometry: width ~2.8, height ~3.95, thickness ~0.015
  const pageGeo = new THREE.BoxGeometry(2.8, 3.95, 0.015)

  const pageData = [
    { title: 'PT TEKNOLOGI NUSANTARA', sub: 'Laporan Magang Divisi Engineering', bab: 'COVER UTAMA', isCover: true, zOffset: 0.08 },
    { title: 'PENDAHULUAN & TUJUAN', sub: '1', bab: 'BAB I', isCover: false, zOffset: 0.04 },
    { title: 'GAMBARAN UMUM PERUSAHAAN', sub: '5', bab: 'BAB II', isCover: false, zOffset: 0.0 },
    { title: 'PELAKSANAAN PRAKTIK KERJA', sub: '11', bab: 'BAB III', isCover: false, zOffset: -0.04 },
    { title: 'PENUTUP & SARAN INDUSTRI', sub: '18', bab: 'BAB IV', isCover: false, zOffset: -0.08 }
  ]

  pageData.forEach((data, i) => {
    const texture = createPageTexture(data.title, data.sub, data.bab, data.isCover)

    const materials = [
      new THREE.MeshStandardMaterial({ color: 0xe2e8f0 }), // right edge
      new THREE.MeshStandardMaterial({ color: 0xe2e8f0 }), // left edge (spine)
      new THREE.MeshStandardMaterial({ color: 0xe2e8f0 }), // top edge
      new THREE.MeshStandardMaterial({ color: 0xe2e8f0 }), // bottom edge
      new THREE.MeshStandardMaterial({ map: texture, roughness: 0.35, metalness: 0.05 }), // front
      new THREE.MeshStandardMaterial({ color: 0xf1f5f9, roughness: 0.5 }) // back
    ]

    const mesh = new THREE.Mesh(pageGeo, materials)
    mesh.position.set(0, 0, data.zOffset)
    mesh.castShadow = true
    mesh.receiveShadow = true
    mesh.userData = { initialZ: data.zOffset, index: i, title: data.title }

    pagesGroup.add(mesh)
    pageMeshes.push(mesh)
  })
}

const buildFloatingNotes = () => {
  const noteGeo = new THREE.PlaneGeometry(1.1, 0.9)
  const noteTexts = [
    '• Install 5 PC Windows 11',
    '• Config IP Static LAN',
    '• Daily Standup 08:30',
    '• Troubleshoot Router Mikrotik',
    '• Deploy Web App ke Server'
  ]

  noteTexts.forEach((text, i) => {
    const canvas = document.createElement('canvas')
    canvas.width = 400
    canvas.height = 300
    const ctx = canvas.getContext('2d')
    if (ctx) {
      ctx.fillStyle = '#1e293b'
      ctx.fillRect(0, 0, 400, 300)
      ctx.strokeStyle = '#38bdf8'
      ctx.lineWidth = 4
      ctx.strokeRect(10, 10, 380, 280)

      ctx.fillStyle = '#38bdf8'
      ctx.font = 'bold 20px monospace'
      ctx.fillText('CATATAN HARIAN #' + (i + 1), 30, 50)

      ctx.fillStyle = '#f8fafc'
      ctx.font = 'bold 22px sans-serif'
      ctx.fillText(text, 30, 140)
    }

    const mat = new THREE.MeshBasicMaterial({
      map: new THREE.CanvasTexture(canvas),
      transparent: true,
      opacity: 0.85,
      side: THREE.DoubleSide
    })

    const noteMesh = new THREE.Mesh(noteGeo, mat)
    const angle = (i / noteTexts.length) * Math.PI * 2
    noteMesh.position.set(Math.cos(angle) * 3.4, Math.sin(angle) * 2.2 + (Math.random() - 0.5), (Math.random() - 0.5) * 1.5)
    noteMesh.rotation.set((Math.random() - 0.5) * 0.4, (Math.random() - 0.5) * 0.4, (Math.random() - 0.5) * 0.4)
    notesGroup.add(noteMesh)
  })
}

// Animate loop with cinematic spring interpolation
const animate = () => {
  animationFrameId = requestAnimationFrame(animate)

  // Smooth lerp mouse coordinates
  mouse.x += (mouse.targetX - mouse.x) * 0.05
  mouse.y += (mouse.targetY - mouse.y) * 0.05

  const time = performance.now() * 0.001

  // Natural subtle floating breathing
  const floatY = Math.sin(time * 1.2) * 0.08
  const floatRot = Math.cos(time * 0.8) * 0.03

  // Base rotation combined with mouse depth parallax
  mainRig.rotation.y = -0.25 + mouse.x * 0.55 + floatRot
  mainRig.rotation.x = 0.15 - mouse.y * 0.45
  mainRig.position.y = floatY

  // Fan out pages or collapse based on mode
  pageMeshes.forEach((mesh, idx) => {
    if (isFanned.value) {
      const spreadX = (idx - 2) * 1.1
      const spreadRotY = (idx - 2) * 0.15
      const spreadZ = (idx - 2) * 0.4

      mesh.position.x += (spreadX - mesh.position.x) * 0.08
      mesh.position.z += (spreadZ - mesh.position.z) * 0.08
      mesh.rotation.y += (spreadRotY - mesh.rotation.y) * 0.08
    } else {
      const targetZ = mesh.userData.initialZ
      mesh.position.x += (0 - mesh.position.x) * 0.08
      mesh.position.z += (targetZ - mesh.position.z) * 0.08
      mesh.rotation.y += (0 - mesh.rotation.y) * 0.08
    }
  })

  // Floating notes gentle orbit
  notesGroup.children.forEach((note, i) => {
    note.position.y += Math.sin(time * 2 + i) * 0.002
    note.rotation.z += 0.001
  })

  renderer.render(scene, camera)
}

const handleMouseMove = (e: MouseEvent) => {
  if (!containerRef.value) return
  const rect = containerRef.value.getBoundingClientRect()
  mouse.targetX = ((e.clientX - rect.left) / rect.width) * 2 - 1
  mouse.targetY = -(((e.clientY - rect.top) / rect.height) * 2 - 1)
}

const toggleFanOut = () => {
  isFanned.value = !isFanned.value
  activeTab.value = isFanned.value ? 'fanned' : 'assembled'
}

const setViewMode = (mode: 'assembled' | 'fanned' | 'notes') => {
  activeTab.value = mode
  if (mode === 'fanned') {
    isFanned.value = true
    notesGroup.visible = true
  } else if (mode === 'assembled') {
    isFanned.value = false
    notesGroup.visible = false
  } else if (mode === 'notes') {
    isFanned.value = false
    notesGroup.visible = true
  }
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
  <div class="relative w-full h-[520px] sm:h-[620px] lg:h-[700px] flex items-center justify-center select-none">
    <!-- 3D WebGL Canvas Viewport -->
    <div
      ref="containerRef"
      class="w-full h-full cursor-grab active:cursor-grabbing transition-transform duration-300"
      @mousemove="handleMouseMove"
      @mouseenter="isHovered = true"
      @mouseleave="() => { isHovered = false; mouse.targetX = 0; mouse.targetY = 0; }"
    />

    <!-- Interactive 3D Perspective Controls Floating HUD -->
    <div class="absolute bottom-4 sm:bottom-6 inset-x-0 flex justify-center items-center gap-2 pointer-events-auto z-20 px-4">
      <div class="bg-[#111620]/90 backdrop-blur-md p-1.5 rounded-xl border border-border/80 shadow-elevated flex items-center gap-1 text-xs">
        <button
          type="button"
          class="px-3 py-1.5 rounded-lg font-medium transition-all cursor-pointer flex items-center gap-1.5"
          :class="activeTab === 'assembled' ? 'bg-accent-500 text-white shadow-subtle' : 'text-ink-secondary hover:text-ink-primary hover:bg-surface-elevated'"
          @click="setViewMode('assembled')"
        >
          <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
          <span>Dokumen Utuh</span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 rounded-lg font-medium transition-all cursor-pointer flex items-center gap-1.5"
          :class="activeTab === 'fanned' ? 'bg-accent-500 text-white shadow-subtle' : 'text-ink-secondary hover:text-ink-primary hover:bg-surface-elevated'"
          @click="setViewMode('fanned')"
        >
          <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
          </svg>
          <span>Bongkar Lembar Bab (3D Fan)</span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 rounded-lg font-medium transition-all cursor-pointer flex items-center gap-1.5"
          :class="activeTab === 'notes' ? 'bg-accent-500 text-white shadow-subtle' : 'text-ink-secondary hover:text-ink-primary hover:bg-surface-elevated'"
          @click="setViewMode('notes')"
        >
          <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
          <span>Transformasi Catatan</span>
        </button>
      </div>
    </div>

    <!-- Atmospheric 3D Lighting Ring Overlay -->
    <div class="absolute inset-0 pointer-events-none bg-radial from-transparent via-transparent to-canvas opacity-70" />
  </div>
</template>
