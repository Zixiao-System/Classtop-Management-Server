<template>
  <div style="margin-top: 24px;">
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
      <h2>客户端列表</h2>
      <mdui-button icon="add" @click="showRegisterDialog">注册新客户端</mdui-button>
    </div>

    <div v-if="clients.length === 0">
      <mdui-card style="padding: 24px; text-align: center;">
        暂无客户端,点击右上角"注册新客户端"添加
      </mdui-card>
    </div>
    <div v-else>
      <mdui-card
        v-for="client in clients"
        :key="client.id"
        class="client-card"
      >
        <div class="client-header">
          <div>
            <h3 style="margin: 0;">{{ client.name }}</h3>
            <div style="color: var(--mdui-color-on-surface-variant); font-size: 0.875rem; margin-top: 4px;">
              UUID: {{ client.uuid }}
            </div>
          </div>
          <div style="display: flex; gap: 8px; align-items: center;">
            <span :class="['status-badge', client.status === 'online' ? 'status-online' : 'status-offline']">
              {{ client.status === 'online' ? '在线' : '离线' }}
            </span>
            <mdui-button-icon icon="visibility--outlined" @click="showClientDetail(client.id)"></mdui-button-icon>
            <mdui-button-icon icon="delete--outlined" @click="deleteClient(client.id)"></mdui-button-icon>
          </div>
        </div>
        <div class="client-info">
          <div><strong>API:</strong> {{ client.api_url }}</div>
          <div><strong>最后同步:</strong> {{ formatDate(client.last_sync) }}</div>
          <div><strong>创建时间:</strong> {{ formatDate(client.created_at) }}</div>
        </div>
        <div v-if="client.description" style="margin-top: 8px; color: var(--mdui-color-on-surface-variant);">
          {{ client.description }}
        </div>
      </mdui-card>
    </div>

    <!-- Register Client Dialog -->
    <mdui-dialog ref="registerDialog" headline="注册新客户端">
      <mdui-text-field
        v-model="registerForm.uuid"
        label="UUID*"
        required
        helper="客户端唯一标识符"
      ></mdui-text-field>
      <mdui-text-field
        v-model="registerForm.name"
        label="名称*"
        required
      ></mdui-text-field>
      <mdui-text-field
        v-model="registerForm.description"
        label="描述"
      ></mdui-text-field>
      <mdui-text-field
        v-model="registerForm.api_url"
        label="API URL*"
        required
      ></mdui-text-field>
      <mdui-text-field
        v-model="registerForm.api_key"
        label="API Key"
        helper="可选"
      ></mdui-text-field>
      <mdui-button slot="action" variant="text" @click="closeRegisterDialog">取消</mdui-button>
      <mdui-button slot="action" variant="text" @click="registerClient">注册</mdui-button>
    </mdui-dialog>

    <!-- Client Detail Dialog -->
    <mdui-dialog ref="detailDialog" headline="客户端详情" style="--mdui-dialog-max-width: 600px;">
      <div v-if="selectedClient">
        <div style="display: grid; gap: 12px;">
          <div><strong>名称:</strong> {{ selectedClient.name }}</div>
          <div><strong>UUID:</strong> {{ selectedClient.uuid }}</div>
          <div>
            <strong>状态:</strong>
            <span :class="['status-badge', selectedClient.status === 'online' ? 'status-online' : 'status-offline']">
              {{ selectedClient.status }}
            </span>
          </div>
          <div><strong>API URL:</strong> {{ selectedClient.api_url }}</div>
          <div v-if="selectedClient.description"><strong>描述:</strong> {{ selectedClient.description }}</div>
          <div><strong>最后同步:</strong> {{ formatDate(selectedClient.last_sync) }}</div>
          <div><strong>创建时间:</strong> {{ formatDate(selectedClient.created_at) }}</div>
        </div>
      </div>
      <mdui-button slot="action" variant="text" @click="closeDetailDialog">关闭</mdui-button>
    </mdui-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { snackbar, confirm } from 'mdui'
import { fetchClients, fetchClient, createClient, removeClient } from '../api'

const clients = ref([])
const selectedClient = ref(null)
const registerDialog = ref(null)
const detailDialog = ref(null)

const registerForm = reactive({
  uuid: '',
  name: '',
  description: '',
  api_url: 'http://localhost:8765',
  api_key: ''
})

onMounted(async () => {
  await loadClients()
})

const loadClients = async () => {
  try {
    clients.value = await fetchClients()
  } catch (error) {
    snackbar({ message: '加载客户端失败: ' + error.message })
  }
}

const showRegisterDialog = () => {
  registerForm.uuid = crypto.randomUUID()
  registerForm.name = ''
  registerForm.description = ''
  registerForm.api_url = 'http://localhost:8765'
  registerForm.api_key = ''
  registerDialog.value.open = true
}

const closeRegisterDialog = () => {
  registerDialog.value.open = false
}

const registerClient = async () => {
  if (!registerForm.uuid || !registerForm.name || !registerForm.api_url) {
    snackbar({ message: '请填写必填项' })
    return
  }

  try {
    const data = {
      uuid: registerForm.uuid,
      name: registerForm.name,
      description: registerForm.description || null,
      api_url: registerForm.api_url,
      api_key: registerForm.api_key || null
    }
    await createClient(data)
    snackbar({ message: '客户端注册成功' })
    closeRegisterDialog()
    await loadClients()
  } catch (error) {
    snackbar({ message: '注册失败: ' + error.message })
  }
}

const deleteClient = async (id) => {
  const result = await confirm({
    headline: '确认删除',
    description: '确定要删除此客户端吗?所有相关数据也将被删除!',
    confirmText: '删除',
    cancelText: '取消'
  })

  if (result) {
    try {
      await removeClient(id)
      snackbar({ message: '客户端删除成功' })
      await loadClients()
    } catch (error) {
      snackbar({ message: '删除失败: ' + error.message })
    }
  }
}

const showClientDetail = async (id) => {
  try {
    selectedClient.value = await fetchClient(id)
    detailDialog.value.open = true
  } catch (error) {
    snackbar({ message: '加载详情失败: ' + error.message })
  }
}

const closeDetailDialog = () => {
  detailDialog.value.open = false
}

const formatDate = (dateString) => {
  if (!dateString) return '未同步'
  return new Date(dateString).toLocaleString('zh-CN')
}
</script>
