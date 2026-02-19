<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { NLayout, NLayoutSider, NLayoutContent, NInput, NButton, NIcon, NEmpty, NSpin, NUpload, NSelect, useMessage } from 'naive-ui';
import type { UploadCustomRequestOptions } from 'naive-ui';
import { Search, Plus, Settings, Star, Tag as TagIcon, LayoutGrid, List, Upload, Box } from 'lucide-vue-next';
import type { FunctionalComponent } from 'vue';
import { usePresetStore } from '../stores/presetStore';
import PresetCard from './PresetCard.vue';
import PresetEditor from './PresetEditor.vue';
import ModelManagement from './ModelManagement.vue';
import type { PresetConfig } from '../types';
import { parseFooocusPresetJson, downloadPresetAsJson } from '../utils/presetConverter';

interface SidebarItem {
  label?: string;
  key: string;
  icon?: FunctionalComponent;
  count?: number;
  color?: string;
  type?: 'divider' | 'section';
}

const store = usePresetStore();
const message = useMessage();

const searchQuery = ref('');
const baseModelFilter = ref<string | null>(null);
const viewMode = ref<'grid' | 'list'>('grid');
const showEditor = ref(false);
const editingPreset = ref<PresetConfig | null>(null);
const collapsed = ref(false);
const activeView = ref<'presets' | 'models'>('presets');

const sidebarItems = computed<SidebarItem[]>(() => [
  { label: '配置管理', key: 'section-presets', type: 'section' },
  { label: '全部配置', key: 'all', icon: LayoutGrid, count: store.presets.length },
  { label: '收藏', key: 'favorites', icon: Star, count: store.favoritePresets.length },
  { type: 'divider', key: 'd1' },
  ...store.tags.map(tag => ({
    label: tag.name,
    key: `tag-${tag.id}`,
    icon: TagIcon,
    count: tag.count,
    color: tag.color,
  })),
  { type: 'divider', key: 'd2' },
  { label: '模型管理', key: 'section-models', type: 'section' },
  { label: '模型信息', key: 'models', icon: Box },
]);

const activeSidebarItem = ref('all');

const handleSidebarClick = (key: string) => {
  if (key === 'models') {
    activeView.value = 'models';
    activeSidebarItem.value = key;
    return;
  }
  
  activeView.value = 'presets';
  activeSidebarItem.value = key;
  
  if (key === 'all') {
    store.setFilter({ tags: [], isFavorite: null });
  } else if (key === 'favorites') {
    store.setFilter({ tags: [], isFavorite: true });
  } else if (key.startsWith('tag-')) {
    const tagName = store.tags.find(t => `tag-${t.id}` === key)?.name;
    if (tagName) {
      store.setFilter({ tags: [tagName], isFavorite: null });
    }
  }
};

const handleSearch = () => {
  store.setFilter({ search: searchQuery.value });
};

const baseModelOptions = computed(() => [
  { label: '全部模型', value: '' },
  ...store.baseModels.map(model => ({ label: model, value: model })),
]);

const handleBaseModelChange = (value: string) => {
  baseModelFilter.value = value || null;
  store.setFilter({ baseModel: value || '' });
};

const handleCreatePreset = () => {
  editingPreset.value = null;
  showEditor.value = true;
};

const handleEditPreset = (preset: PresetConfig) => {
  editingPreset.value = preset;
  showEditor.value = true;
};

const handleEditorClose = () => {
  showEditor.value = false;
  editingPreset.value = null;
};

const handleEditorSave = async (preset: PresetConfig) => {
  if (editingPreset.value) {
    await store.updatePreset(preset);
  } else {
    await store.createPreset(preset);
  }
  showEditor.value = false;
  editingPreset.value = null;
};

const handleDeletePreset = async (id: string) => {
  if (confirm('确定要删除这个配置吗？')) {
    await store.deletePreset(id);
  }
};

const handleToggleFavorite = async (id: string) => {
  await store.toggleFavorite(id);
};

const handleExportPreset = async (preset: PresetConfig) => {
  await downloadPresetAsJson(preset);
  message.success('导出成功');
};

const handleImportPreset = async ({ file }: UploadCustomRequestOptions) => {
  try {
    const fileObj = file.file;
    if (!fileObj) return;
    const text = await fileObj.text();
    const preset = parseFooocusPresetJson(text);
    const created = await store.createPreset(preset);
    if (created) {
      message.success('导入成功');
    }
  } catch (e) {
    message.error('导入失败：文件格式不正确');
    console.error('Import error:', e);
  }
};

onMounted(() => {
  store.fetchPresets();
  store.fetchTags();
});
</script>

<template>
  <NLayout has-sider class="h-screen">
    <NLayoutSider
      bordered
      :collapsed="collapsed"
      collapse-mode="width"
      :collapsed-width="64"
      :width="240"
      :native-scrollbar="false"
      class="bg-gray-50 dark:bg-gray-900"
    >
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-2">
          <div class="w-8 h-8 bg-indigo-500 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold text-sm">F</span>
          </div>
          <span v-if="!collapsed" class="font-semibold text-gray-800 dark:text-gray-200">
            Fooocus 配置管理器
          </span>
        </div>
      </div>

      <div class="p-2">
        <div
          v-for="item in sidebarItems"
          :key="item.key"
          class="mb-1"
        >
          <div
            v-if="item.type === 'section'"
            class="px-3 py-2 text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wider"
          >
            {{ item.label }}
          </div>
          <div
            v-else-if="item.type === 'divider'"
            class="my-2 border-t border-gray-200 dark:border-gray-700"
          />
          <div
            v-else
            class="flex items-center gap-3 px-3 py-2 rounded-lg cursor-pointer transition-colors"
            :class="[
              activeSidebarItem === item.key
                ? 'bg-indigo-100 dark:bg-indigo-900 text-indigo-600 dark:text-indigo-300'
                : 'hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-300'
            ]"
            @click="handleSidebarClick(item.key)"
          >
            <component
              v-if="item.icon"
              :is="item.icon"
              :size="18"
              :style="item.color ? { color: item.color } : {}"
            />
            <span v-if="!collapsed" class="flex-1 text-sm">{{ item.label }}</span>
            <span
              v-if="!collapsed && item.count !== undefined"
              class="text-xs text-gray-500 dark:text-gray-400"
            >
              {{ item.count }}
            </span>
          </div>
        </div>
      </div>

      <div v-if="!collapsed" class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
        <NButton quaternary size="small" class="w-full justify-start">
          <template #icon>
            <NIcon :component="Settings" />
          </template>
          设置
        </NButton>
      </div>
    </NLayoutSider>

    <NLayoutContent :native-scrollbar="false" class="bg-white dark:bg-gray-800">
      <ModelManagement v-if="activeView === 'models'" />
      
      <div v-else class="h-full flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center gap-4">
          <div class="flex-1">
            <NInput
              v-model:value="searchQuery"
              placeholder="搜索配置..."
              clearable
              @update:value="handleSearch"
              style="min-width: 200px"
            >
              <template #prefix>
                <NIcon :component="Search" />
              </template>
            </NInput>
          </div>

          <NSelect
            v-model:value="baseModelFilter"
            :options="baseModelOptions"
            placeholder="基础模型"
            clearable
            style="min-width: 150px"
            @update:value="handleBaseModelChange"
          />

          <div class="flex items-center gap-2">
            <NButton
              :type="viewMode === 'grid' ? 'primary' : 'default'"
              quaternary
              circle
              @click="viewMode = 'grid'"
            >
              <template #icon>
                <NIcon :component="LayoutGrid" />
              </template>
            </NButton>
            <NButton
              :type="viewMode === 'list' ? 'primary' : 'default'"
              quaternary
              circle
              @click="viewMode = 'list'"
            >
              <template #icon>
                <NIcon :component="List" />
              </template>
            </NButton>
          </div>

          <NUpload
            accept=".json"
            :show-file-list="false"
            :custom-request="handleImportPreset"
          >
            <NButton>
              <template #icon>
                <NIcon :component="Upload" />
              </template>
              导入
            </NButton>
          </NUpload>

          <NButton type="primary" @click="handleCreatePreset">
            <template #icon>
              <NIcon :component="Plus" />
            </template>
            新建配置
          </NButton>
        </div>

        <div class="flex-1 overflow-auto p-4">
          <NSpin :show="store.isLoading">
            <div v-if="store.filteredPresets.length === 0" class="h-full flex items-center justify-center">
              <NEmpty description="暂无配置">
                <template #extra>
                  <NButton size="small" @click="handleCreatePreset">
                    创建第一个配置
                  </NButton>
                </template>
              </NEmpty>
            </div>

            <div
              v-else
              :class="[
                viewMode === 'grid'
                  ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4'
                  : 'flex flex-col gap-2 w-full'
              ]"
            >
              <PresetCard
                v-for="preset in store.filteredPresets"
                :key="preset.id"
                :preset="preset"
                :view-mode="viewMode"
                @edit="handleEditPreset"
                @delete="handleDeletePreset"
                @toggle-favorite="handleToggleFavorite"
                @export="handleExportPreset"
              />
            </div>
          </NSpin>
        </div>
      </div>
    </NLayoutContent>

    <PresetEditor
      v-if="showEditor"
      :preset="editingPreset"
      @close="handleEditorClose"
      @save="handleEditorSave"
    />
  </NLayout>
</template>
