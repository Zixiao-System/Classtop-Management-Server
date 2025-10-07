<template>
  <div style="margin-top: 24px;">
    <h2>数据查看</h2>
    <mdui-select
      v-model="selectedClientId"
      label="选择客户端"
      @change="loadClientData"
    >
      <mdui-menu-item value="">-- 选择客户端 --</mdui-menu-item>
      <mdui-menu-item
        v-for="client in clients"
        :key="client.id"
        :value="String(client.id)"
      >
        {{ client.name }}
      </mdui-menu-item>
    </mdui-select>

    <div v-if="selectedClientId" style="margin-top: 24px;">
      <h3>课程列表</h3>
      <div v-if="courses.length === 0">
        <mdui-card style="padding: 16px;">暂无课程数据</mdui-card>
      </div>
      <div v-else>
        <mdui-card
          v-for="course in courses"
          :key="course.id"
          style="padding: 12px; margin-bottom: 8px;"
          :style="{ borderLeft: `4px solid ${course.color || '#6750A4'}` }"
        >
          <div style="font-weight: bold;">{{ course.name }}</div>
          <div v-if="course.teacher" style="font-size: 0.875rem; margin-top: 4px;">
            教师: {{ course.teacher }}
          </div>
        </mdui-card>
      </div>
    </div>

    <div v-if="selectedClientId" style="margin-top: 24px;">
      <h3>课程表</h3>
      <div v-if="schedule.length === 0">
        <mdui-card style="padding: 16px;">暂无课程表数据</mdui-card>
      </div>
      <div v-else style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px;">
        <mdui-card v-for="(entries, day) in groupedSchedule" :key="day">
          <div style="padding: 16px;">
            <h4 style="margin: 0 0 12px 0;">{{ days[day - 1] }}</h4>
            <div v-if="entries.length === 0" style="color: var(--mdui-color-on-surface-variant);">
              无课程
            </div>
            <div
              v-for="entry in entries"
              :key="entry.id"
              style="padding: 8px; margin-bottom: 8px; background: var(--mdui-color-surface-variant); border-radius: 4px;"
              :style="{ borderLeft: `3px solid ${entry.color || '#6750A4'}` }"
            >
              <div style="font-weight: bold;">{{ entry.course_name }}</div>
              <div style="font-size: 0.875rem;">{{ entry.start_time }} - {{ entry.end_time }}</div>
            </div>
          </div>
        </mdui-card>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { snackbar } from 'mdui'
import { fetchClients, fetchClientCourses, fetchClientSchedule } from '../api'

const clients = ref([])
const selectedClientId = ref('')
const courses = ref([])
const schedule = ref([])

const days = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']

const groupedSchedule = computed(() => {
  const grouped = {}
  days.forEach((_, i) => {
    grouped[i + 1] = []
  })
  schedule.value.forEach(entry => {
    if (grouped[entry.day_of_week]) {
      grouped[entry.day_of_week].push(entry)
    }
  })
  return grouped
})

onMounted(async () => {
  await loadClients()
})

const loadClients = async () => {
  try {
    clients.value = await fetchClients()
  } catch (error) {
    snackbar({ message: '加载客户端列表失败: ' + error.message })
  }
}

const loadClientData = async () => {
  if (!selectedClientId.value) return

  try {
    const [coursesData, scheduleData] = await Promise.all([
      fetchClientCourses(selectedClientId.value),
      fetchClientSchedule(selectedClientId.value)
    ])
    courses.value = coursesData
    schedule.value = scheduleData
  } catch (error) {
    snackbar({ message: '加载数据失败: ' + error.message })
  }
}
</script>
