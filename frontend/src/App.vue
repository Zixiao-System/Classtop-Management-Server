<template>
  <div v-if="!isAuthenticated">
    <!-- 登录界面 -->
    <LoginView @login-success="onLoginSuccess" />
  </div>

  <div v-else>
    <!-- Top App Bar -->
    <mdui-top-app-bar>
      <mdui-top-app-bar-title>ClassTop 集中管理服务器</mdui-top-app-bar-title>
      <div style="flex-grow: 1"></div>

      <!-- 用户信息 -->
      <div style="margin-right: 16px; display: flex; align-items: center; gap: 8px;">
        <mdui-icon name="account_circle"></mdui-icon>
        <span style="font-size: 0.875rem;">{{ currentUser?.username }}</span>
      </div>

      <mdui-button-icon icon="refresh--outlined" @click="refreshData"></mdui-button-icon>
      <mdui-button-icon
        :icon="isDark ? 'light_mode--outlined' : 'dark_mode--outlined'"
        @click="toggleTheme"
      ></mdui-button-icon>
      <mdui-button-icon icon="settings--outlined" @click="openApiDocs"></mdui-button-icon>
      <mdui-button-icon icon="logout" @click="handleLogout"></mdui-button-icon>
    </mdui-top-app-bar>

    <!-- Navigation Tabs -->
    <mdui-tabs v-model="currentTab" style="position: sticky; top: 64px; background: var(--mdui-color-surface); z-index: 100;">
      <mdui-tab value="dashboard">仪表板</mdui-tab>
      <mdui-tab value="clients">客户端管理</mdui-tab>
      <mdui-tab value="data">数据查看</mdui-tab>
    </mdui-tabs>

    <div class="container">
      <!-- Dashboard Tab -->
      <DashboardView v-if="currentTab === 'dashboard'" />

      <!-- Clients Tab -->
      <ClientsView v-if="currentTab === 'clients'" />

      <!-- Data Tab -->
      <DataView v-if="currentTab === 'data'" />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { setTheme, getTheme, snackbar, confirm } from 'mdui'
import { auth } from './auth.js'
import LoginView from './components/LoginView.vue'
import DashboardView from './components/DashboardView.vue'
import ClientsView from './components/ClientsView.vue'
import DataView from './components/DataView.vue'

const currentTab = ref('dashboard')
const isDark = ref(false)
const isAuthenticated = ref(false)
const currentUser = computed(() => auth.getUser())

onMounted(() => {
  // 检查认证状态
  isAuthenticated.value = auth.isAuthenticated()

  // 设置主题
  const theme = getTheme()
  isDark.value = theme === 'dark' || (theme === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)
})

const onLoginSuccess = () => {
  isAuthenticated.value = true
  snackbar({
    message: `欢迎回来，${currentUser.value?.username}！`,
    action: '确定'
  })
}

const toggleTheme = () => {
  const currentTheme = getTheme()
  if (currentTheme === 'dark') {
    setTheme('light')
    isDark.value = false
  } else {
    setTheme('dark')
    isDark.value = true
  }
}

const refreshData = () => {
  window.location.reload()
}

const openApiDocs = () => {
  window.open('/api/docs', '_blank')
}

const handleLogout = async () => {
  const result = await confirm({
    headline: '确认登出',
    description: '确定要退出登录吗？',
    confirmText: '确定',
    cancelText: '取消'
  })

  if (result) {
    auth.logout()
    isAuthenticated.value = false
    snackbar({
      message: '已退出登录',
      action: '确定'
    })
  }
}
</script>

<style>
body {
  margin: 0;
  padding: 0;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 16px;
  margin-bottom: 32px;
}

.stat-card {
  text-align: center;
  padding: 24px;
}

.stat-value {
  font-size: 2.5rem;
  font-weight: bold;
  margin: 8px 0;
}

.client-card {
  margin-bottom: 16px;
  padding: 16px;
}

.client-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 0.875rem;
}

.status-online {
  background: #4caf50;
  color: white;
}

.status-offline {
  background: #9e9e9e;
  color: white;
}

.client-info {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 8px;
  font-size: 0.875rem;
}
</style>
