<template>
  <div class="login-container">
    <mdui-card class="login-card">
      <div class="login-header">
        <mdui-icon name="admin_panel_settings" style="font-size: 48px; color: var(--mdui-color-primary);"></mdui-icon>
        <h2>ClassTop 管理系统</h2>
        <p>{{ isLogin ? '登录到管理面板' : '创建管理员账户' }}</p>
      </div>

      <mdui-tabs v-model="activeTab" @change="onTabChange">
        <mdui-tab value="login">登录</mdui-tab>
        <mdui-tab value="register">注册</mdui-tab>
      </mdui-tabs>

      <div class="login-form">
        <!-- 登录表单 -->
        <div v-if="isLogin">
          <mdui-text-field
            v-model="loginForm.username"
            label="用户名"
            placeholder="请输入用户名"
            required
            @keyup.enter="handleLogin"
          ></mdui-text-field>

          <mdui-text-field
            v-model="loginForm.password"
            type="password"
            label="密码"
            placeholder="请输入密码"
            required
            toggle-password
            @keyup.enter="handleLogin"
          ></mdui-text-field>

          <mdui-button
            full-width
            variant="filled"
            @click="handleLogin"
            :loading="loading"
            style="margin-top: 24px;"
          >
            登录
          </mdui-button>
        </div>

        <!-- 注册表单 -->
        <div v-else>
          <mdui-text-field
            v-model="registerForm.username"
            label="用户名"
            placeholder="请输入用户名"
            required
            helper="用户名用于登录系统"
          ></mdui-text-field>

          <mdui-text-field
            v-model="registerForm.email"
            type="email"
            label="邮箱（可选）"
            placeholder="请输入邮箱"
            helper="用于找回密码"
          ></mdui-text-field>

          <mdui-text-field
            v-model="registerForm.password"
            type="password"
            label="密码"
            placeholder="请输入密码"
            required
            toggle-password
            helper="密码长度至少 6 位"
          ></mdui-text-field>

          <mdui-text-field
            v-model="registerForm.confirmPassword"
            type="password"
            label="确认密码"
            placeholder="请再次输入密码"
            required
            toggle-password
            @keyup.enter="handleRegister"
          ></mdui-text-field>

          <mdui-button
            full-width
            variant="filled"
            @click="handleRegister"
            :loading="loading"
            style="margin-top: 24px;"
          >
            注册
          </mdui-button>
        </div>

        <!-- 错误提示 -->
        <mdui-linear-progress v-if="loading" style="margin-top: 16px;"></mdui-linear-progress>

        <div v-if="error" class="error-message">
          <mdui-icon name="error"></mdui-icon>
          <span>{{ error }}</span>
        </div>
      </div>
    </mdui-card>

    <div class="login-footer">
      <p>ClassTop Management Server v1.1.0</p>
      <p>
        <a href="/api/docs" target="_blank" style="color: var(--mdui-color-primary); text-decoration: none;">API 文档</a>
      </p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { snackbar } from 'mdui'
import { auth } from '../auth.js'

const router = useRouter()

const activeTab = ref('login')
const loading = ref(false)
const error = ref('')

const loginForm = ref({
  username: '',
  password: ''
})

const registerForm = ref({
  username: '',
  email: '',
  password: '',
  confirmPassword: ''
})

const isLogin = computed(() => activeTab.value === 'login')

const onTabChange = () => {
  error.value = ''
}

const handleLogin = async () => {
  error.value = ''

  if (!loginForm.value.username || !loginForm.value.password) {
    error.value = '请填写用户名和密码'
    return
  }

  loading.value = true

  try {
    await auth.login(loginForm.value.username, loginForm.value.password)
    snackbar({
      message: '登录成功！',
      action: '确定'
    })
    router.push('/')
  } catch (err) {
    error.value = err.message || '登录失败，请检查用户名和密码'
    snackbar({
      message: error.value,
      action: '关闭'
    })
  } finally {
    loading.value = false
  }
}

const handleRegister = async () => {
  error.value = ''

  if (!registerForm.value.username || !registerForm.value.password) {
    error.value = '请填写用户名和密码'
    return
  }

  if (registerForm.value.password.length < 6) {
    error.value = '密码长度至少 6 位'
    return
  }

  if (registerForm.value.password !== registerForm.value.confirmPassword) {
    error.value = '两次输入的密码不一致'
    return
  }

  loading.value = true

  try {
    await auth.register(
      registerForm.value.username,
      registerForm.value.password,
      registerForm.value.email || null
    )
    snackbar({
      message: '注册成功！正在登录...',
      action: '确定'
    })
    router.push('/')
  } catch (err) {
    error.value = err.message || '注册失败，用户名可能已被使用'
    snackbar({
      message: error.value,
      action: '关闭'
    })
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 100%;
  max-width: 450px;
  padding: 32px;
}

.login-header {
  text-align: center;
  margin-bottom: 24px;
}

.login-header h2 {
  margin: 16px 0 8px;
  font-size: 1.75rem;
}

.login-header p {
  margin: 0;
  color: var(--mdui-color-on-surface-variant);
}

.login-form {
  margin-top: 24px;
}

.login-form mdui-text-field {
  margin-bottom: 16px;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 16px;
  padding: 12px;
  background: var(--mdui-color-error-container);
  color: var(--mdui-color-on-error-container);
  border-radius: 8px;
  font-size: 0.875rem;
}

.login-footer {
  margin-top: 24px;
  text-align: center;
  color: rgba(255, 255, 255, 0.9);
}

.login-footer p {
  margin: 8px 0;
  font-size: 0.875rem;
}
</style>
