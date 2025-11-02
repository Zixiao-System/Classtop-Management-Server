<template>
  <mdui-layout>
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

    <mdui-layout-main class="main-container">
      <div class="content">
        <transition name="fade">
          <keep-alive>
            <router-view></router-view>
          </keep-alive>
        </transition>
      </div>
    </mdui-layout-main>

    <mdui-navigation-bar :value="selectedItem" @change="routeTo($event.target.value)">
      <mdui-navigation-bar-item v-for="item in items" :key="item.value" :icon="item.icon" :value="item.value">
        {{ item.label }}
      </mdui-navigation-bar-item>
    </mdui-navigation-bar>
  </mdui-layout>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { setTheme, getTheme, snackbar, confirm } from 'mdui'
import { auth } from './auth.js'

const router = useRouter()
const route = useRoute()

const items = [
  { icon: 'dashboard', value: '/', label: '仪表板' },
  { icon: 'devices', value: '/clients', label: '客户端' },
  { icon: 'storage', value: '/data', label: '数据' },
]

const selectedItem = ref('/')
const isDark = ref(false)
const currentUser = computed(() => auth.getUser())

onMounted(() => {
  // 设置主题
  const theme = getTheme()
  isDark.value = theme === 'dark' || (theme === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)

  // 同步当前路由到导航栏
  selectedItem.value = route.path
})

// 监听路由变化，更新选中项
watch(() => route.path, (newPath) => {
  selectedItem.value = newPath
})

const routeTo = (value) => {
  selectedItem.value = value
  router.push(value)
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
    router.push('/login')
    snackbar({
      message: '已退出登录',
      action: '确定'
    })
  }
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--mdui-motion-duration-medium3) var(--mdui-motion-easing-emphasized);
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.main-container {
  display: flex;
  justify-content: left;
  align-items: start;
  height: 100%;
  padding-top: 64px; /* Top app bar height */
}

.content {
  flex: 1;
  overflow: auto;
  height: 100%;
  width: 100%;
  padding: 1rem;
  max-width: 1400px;
  margin: 0 auto;
}

mdui-layout {
  height: 100%;
}
</style>