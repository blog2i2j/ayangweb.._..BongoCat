<script setup lang="ts">
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { Switch } from 'ant-design-vue'
import { watch } from 'vue'

import MacosPermissions from './components/macos-permissions/index.vue'

import ProList from '@/components/pro-list/index.vue'
import ProListItem from '@/components/pro-list-item/index.vue'
import { useGeneralStore } from '@/stores/general'

const generalStore = useGeneralStore()

watch(() => generalStore.autostart, async (value) => {
  const enabled = await isEnabled()

  if (value && !enabled) {
    return enable()
  }

  if (!value && enabled) {
    disable()
  }
}, { immediate: true })
</script>

<template>
  <MacosPermissions />

  <ProList title="应用设置">
    <ProListItem title="开机自启动">
      <Switch v-model:checked="generalStore.autostart" />
    </ProListItem>

    <ProListItem
      description="启用后，即可通过 OBS Studio 捕获窗口。"
      title="显示任务栏图标"
    >
      <Switch v-model:checked="generalStore.taskbarVisibility" />
    </ProListItem>
  </ProList>

  <ProList title="更新设置">
    <ProListItem title="自动检查更新">
      <Switch v-model:checked="generalStore.autoCheckUpdate" />
    </ProListItem>
  </ProList>
</template>
