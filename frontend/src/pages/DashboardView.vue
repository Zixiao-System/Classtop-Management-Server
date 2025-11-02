<template>
  <div style="margin-top: 24px;">
    <h2>服务器概览</h2>
    <div class="stats-grid">
      <mdui-card class="stat-card">
        <div>客户端总数</div>
        <div class="stat-value">{{ stats.total_clients }}</div>
      </mdui-card>
      <mdui-card class="stat-card">
        <div>在线客户端</div>
        <div class="stat-value" style="color: #4caf50;">{{ stats.online_clients }}</div>
      </mdui-card>
      <mdui-card class="stat-card">
        <div>课程总数</div>
        <div class="stat-value">{{ stats.total_courses }}</div>
      </mdui-card>
      <mdui-card class="stat-card">
        <div>课程表条目</div>
        <div class="stat-value">{{ stats.total_schedule_entries }}</div>
      </mdui-card>
    </div>

    <h3>客户端统计</h3>
    <div v-if="clientStats.length === 0">
      <mdui-card style="padding: 24px; text-align: center;">暂无客户端数据</mdui-card>
    </div>
    <div v-else>
      <mdui-card
        v-for="stat in clientStats"
        :key="stat.client_id"
        class="client-card"
      >
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <div>
            <div style="font-weight: bold; font-size: 1.1rem;">{{ stat.client_name }}</div>
            <div style="margin-top: 8px; display: flex; gap: 16px; font-size: 0.875rem; color: var(--mdui-color-on-surface-variant);">
              <span>📚 {{ stat.total_courses }} 门课程</span>
              <span>📅 {{ stat.total_schedule_entries }} 个课表</span>
              <span>🕐 {{ formatDate(stat.last_sync) }}</span>
            </div>
          </div>
        </div>
      </mdui-card>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { snackbar } from 'mdui'
import { fetchStats, fetchClientStats } from '../api'

const stats = ref({
  total_clients: 0,
  online_clients: 0,
  total_courses: 0,
  total_schedule_entries: 0
})
const clientStats = ref([])

onMounted(async () => {
  await loadData()
})

const loadData = async () => {
  try {
    const [statsData, clientStatsData] = await Promise.all([
      fetchStats(),
      fetchClientStats()
    ])
    stats.value = statsData
    clientStats.value = clientStatsData
  } catch (error) {
    snackbar({ message: '加载数据失败: ' + error.message })
  }
}

const formatDate = (dateString) => {
  if (!dateString) return '未同步'
  return new Date(dateString).toLocaleString('zh-CN')
}
</script>
