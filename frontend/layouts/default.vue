<script setup lang="ts">
const { initAuth } = useAuth()

onMounted(() => {
  initAuth()
})
</script>

<template>
  <div class="min-h-screen flex flex-col bg-canvas text-ink-primary antialiased">
    <AppHeader />
    <main class="flex-1 flex flex-col">
      <slot />
    </main>
    <AppFooter />
    <ToastContainer />
  </div>
</template>
