<script setup lang="ts">
import type { UserResponse } from '~/types/api'

definePageMeta({
  middleware: 'admin'
})

const { getUsers, updateUserRole, updateUserStatus, deleteUser } = useAdmin()
const { authState } = useAuth()
const toast = useToast()

const users = ref<UserResponse[]>([])
const total = ref(0)
const page = ref(1)
const totalPages = ref(1)
const limit = ref(20)
const searchQuery = ref('')
const loading = ref(true)
const actionInProgress = ref<string | null>(null)

const loadUsers = async () => {
  loading.value = true
  try {
    const data = await getUsers(page.value, limit.value, searchQuery.value.trim() || undefined)
    users.value = data.users
    total.value = data.total
    totalPages.value = data.total_pages || 1
  } catch (err: any) {
    toast.error('Gagal Memuat Pengguna', err.message || 'Terjadi kesalahan.')
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  page.value = 1
  loadUsers()
}

const handlePageChange = (newPage: number) => {
  if (newPage < 1 || newPage > totalPages.value) return
  page.value = newPage
  loadUsers()
}

const handleToggleRole = async (user: UserResponse) => {
  const newRole = user.role === 'admin' ? 'user' : 'admin'
  if (user.id === authState.value.userId && newRole === 'user') {
    if (!confirm('Peringatan: Anda akan mencabut status Admin Anda sendiri. Lanjutkan?')) {
      return
    }
  }

  actionInProgress.value = user.id
  try {
    await updateUserRole(user.id, newRole)
    user.role = newRole
  } catch (err: any) {
    toast.error('Gagal Mengubah Role', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const handleToggleStatus = async (user: UserResponse) => {
  const newStatus = !user.is_active
  actionInProgress.value = user.id
  try {
    await updateUserStatus(user.id, newStatus)
    user.is_active = newStatus
  } catch (err: any) {
    toast.error('Gagal Mengubah Status', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const handleDeleteUser = async (user: UserResponse) => {
  if (user.id === authState.value.userId) {
    toast.error('Tidak Diizinkan', 'Anda tidak dapat menghapus akun Anda sendiri.')
    return
  }

  if (!confirm(`Apakah Anda yakin ingin menghapus pengguna ${user.email}? Semua data laporan pengguna ini akan terhapus.`)) {
    return
  }

  actionInProgress.value = user.id
  try {
    await deleteUser(user.id)
    await loadUsers()
  } catch (err: any) {
    toast.error('Gagal Menghapus Pengguna', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const formatDate = (iso: string) => {
  if (!iso) return '-'
  const d = new Date(iso)
  return d.toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  })
}

onMounted(() => {
  loadUsers()
})
</script>

<template>
  <div class="min-h-screen bg-canvas pb-16">
    <AdminNav />

    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
        <div>
          <h2 class="text-xl font-bold text-ink-primary">Manajemen Pengguna</h2>
          <p class="text-xs sm:text-sm text-ink-secondary mt-0.5">
            Daftar seluruh akun terdaftar di sistem. Anda dapat mempromosikan admin, menonaktifkan akun, atau menghapus pengguna.
          </p>
        </div>
      </div>

      <!-- Search Bar -->
      <div class="bg-surface border border-border p-4 rounded-xl mb-6 flex items-center justify-between">
        <div class="w-full max-w-md relative">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Cari email pengguna..."
            class="w-full pl-9 pr-4 py-2 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary placeholder:text-ink-muted focus:outline-none focus:border-accent-500"
            @keyup.enter="handleSearch"
          />
          <svg class="w-4 h-4 text-ink-muted absolute left-3 top-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>

        <button
          type="button"
          class="px-4 py-2 text-xs font-medium bg-surface-elevated hover:bg-surface-hover border border-border text-ink-primary rounded-lg transition-colors"
          @click="handleSearch"
        >
          Cari
        </button>
      </div>

      <!-- Users Table -->
      <div class="bg-surface border border-border rounded-xl overflow-hidden shadow-subtle">
        <div v-if="loading" class="p-12 text-center">
          <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
          <p class="text-xs text-ink-muted mt-2 font-mono">Memuat daftar pengguna...</p>
        </div>

        <div v-else-if="users.length === 0" class="p-12 text-center text-ink-muted">
          <div class="text-sm font-medium text-ink-primary">Tidak ada pengguna ditemukan</div>
          <div class="text-xs text-ink-muted mt-0.5">Coba sesuaikan kata kunci pencarian.</div>
        </div>

        <div v-else class="overflow-x-auto">
          <table class="w-full text-left border-collapse text-xs">
            <thead>
              <tr class="border-b border-border bg-surface-elevated/80 font-mono text-[11px] text-ink-muted uppercase">
                <th class="py-3 px-4">Email</th>
                <th class="py-3 px-4">Role</th>
                <th class="py-3 px-4">Jumlah Laporan</th>
                <th class="py-3 px-4">Status Akun</th>
                <th class="py-3 px-4">Terdaftar Sejak</th>
                <th class="py-3 px-4 text-right">Aksi</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border/60">
              <tr v-for="u in users" :key="u.id" class="hover:bg-surface-elevated/40 transition-colors">
                <!-- Email -->
                <td class="py-3 px-4 font-mono font-medium text-ink-primary">
                  <div class="flex items-center gap-2">
                    <span>{{ u.email }}</span>
                    <span v-if="u.id === authState.userId" class="text-[10px] bg-accent-500/15 text-accent-400 px-1.5 py-0.5 rounded font-sans font-bold">
                      Anda
                    </span>
                  </div>
                  <span class="text-[10px] text-ink-muted font-normal block">{{ u.id }}</span>
                </td>

                <!-- Role -->
                <td class="py-3 px-4">
                  <span
                    class="px-2 py-0.5 text-[11px] font-mono rounded font-semibold uppercase border inline-flex items-center gap-1"
                    :class="[
                      u.role === 'admin'
                        ? 'bg-amber-500/15 text-amber-300 border-amber-500/30'
                        : 'bg-surface-elevated text-ink-muted border-border'
                    ]"
                  >
                    <span v-if="u.role === 'admin'" class="w-1.5 h-1.5 rounded-full bg-amber-400" />
                    {{ u.role }}
                  </span>
                </td>

                <!-- Reports Count -->
                <td class="py-3 px-4 font-mono">
                  {{ u.report_count ?? 0 }} laporan
                </td>

                <!-- Status -->
                <td class="py-3 px-4">
                  <span
                    class="px-2 py-0.5 text-[10px] font-mono rounded border inline-block"
                    :class="[
                      u.is_active
                        ? 'bg-accent-500/10 text-accent-400 border-accent-500/20'
                        : 'bg-rose-500/10 text-rose-400 border-rose-500/20'
                    ]"
                  >
                    {{ u.is_active ? 'Aktif' : 'Nonaktif' }}
                  </span>
                </td>

                <!-- Created Date -->
                <td class="py-3 px-4 text-ink-muted">
                  {{ formatDate(u.created_at) }}
                </td>

                <!-- Actions -->
                <td class="py-3 px-4 text-right">
                  <div class="flex items-center justify-end gap-1.5">
                    <!-- Toggle Role Button -->
                    <button
                      type="button"
                      :disabled="actionInProgress === u.id"
                      class="px-2.5 py-1 text-[11px] rounded font-medium transition-colors"
                      :class="[
                        u.role === 'admin'
                          ? 'bg-surface-elevated text-ink-secondary hover:text-ink-primary border border-border'
                          : 'bg-amber-500/15 text-amber-300 hover:bg-amber-500/25 border border-amber-500/30'
                      ]"
                      :title="u.role === 'admin' ? 'Ubah menjadi user biasa' : 'Jadikan admin sistem'"
                      @click="handleToggleRole(u)"
                    >
                      {{ u.role === 'admin' ? 'Demote ke User' : 'Jadikan Admin' }}
                    </button>

                    <!-- Toggle Status Button -->
                    <button
                      type="button"
                      :disabled="actionInProgress === u.id"
                      class="p-1 text-ink-muted hover:text-ink-primary hover:bg-surface-elevated rounded transition-colors"
                      :title="u.is_active ? 'Nonaktifkan Akun' : 'Aktifkan Akun'"
                      @click="handleToggleStatus(u)"
                    >
                      <svg v-if="u.is_active" class="w-4 h-4 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
                      </svg>
                      <svg v-else class="w-4 h-4 text-accent-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                      </svg>
                    </button>

                    <!-- Delete Button -->
                    <button
                      type="button"
                      :disabled="actionInProgress === u.id || u.id === authState.userId"
                      class="p-1 text-ink-muted hover:text-rose-400 hover:bg-rose-500/10 rounded transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                      title="Hapus Pengguna"
                      @click="handleDeleteUser(u)"
                    >
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Pagination Bar -->
        <div class="p-4 border-t border-border flex flex-col sm:flex-row items-center justify-between gap-3 text-xs text-ink-muted">
          <div>
            Menampilkan {{ users.length }} dari {{ total }} total pengguna (Halaman {{ page }} dari {{ totalPages }})
          </div>

          <div class="flex items-center gap-1.5">
            <button
              type="button"
              :disabled="page <= 1"
              class="px-2.5 py-1 rounded bg-surface-elevated border border-border disabled:opacity-40 disabled:cursor-not-allowed hover:bg-surface-hover text-ink-primary"
              @click="handlePageChange(page - 1)"
            >
              &larr; Sebelumnya
            </button>
            <span class="px-2 font-mono">{{ page }}</span>
            <button
              type="button"
              :disabled="page >= totalPages"
              class="px-2.5 py-1 rounded bg-surface-elevated border border-border disabled:opacity-40 disabled:cursor-not-allowed hover:bg-surface-hover text-ink-primary"
              @click="handlePageChange(page + 1)"
            >
              Selanjutnya &rarr;
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
