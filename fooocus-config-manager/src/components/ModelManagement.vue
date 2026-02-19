<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { NInput, NButton, NIcon, NEmpty, NSpin, useMessage } from 'naive-ui';
import { Search, Plus, LayoutGrid, List, Box, Layers, Sparkles, FileCode } from 'lucide-vue-next';
import type { FunctionalComponent } from 'vue';
import { useModelStore } from '../stores/modelStore';
import ModelCard from './ModelCard.vue';
import ModelEditor from './ModelEditor.vue';
import type { ModelInfo, ModelType } from '../types';

interface TypeTab {
  label: string;
  key: ModelType | 'all';
  icon: FunctionalComponent;
}

const store = useModelStore();
const message = useMessage();

const searchQuery = ref('');
const viewMode = ref<'grid' | 'list'>('grid');
const showEditor = ref(false);
const editingModel = ref<ModelInfo | null>(null);
const activeType = ref<ModelType | 'all'>('all');

const typeTabs: TypeTab[] = [
  { label: '全部', key: 'all', icon: Box },
  { label: 'Checkpoint', key: 'Checkpoint', icon: Box },
  { label: 'LoRA', key: 'LoRA', icon: Layers },
  { label: 'Refiner', key: 'Refiner', icon: Sparkles },
  { label: 'Embedding', key: 'Embedding', icon: FileCode },
];

const handleTypeChange = (type: ModelType | 'all') => {
  activeType.value = type;
  store.setFilter({ type: type === 'all' ? '' : type });
};

const handleSearch = () => {
  store.setFilter({ search: searchQuery.value });
};

const handleCreateModel = () => {
  editingModel.value = null;
  showEditor.value = true;
};

const handleEditModel = (model: ModelInfo) => {
  editingModel.value = model;
  showEditor.value = true;
};

const handleEditorClose = () => {
  showEditor.value = false;
  editingModel.value = null;
};

const handleEditorSave = async (model: ModelInfo) => {
  if (editingModel.value) {
    const updated = await store.updateModel(model);
    if (updated) {
      message.success('模型更新成功');
    }
  } else {
    const created = await store.createModel(model);
    if (created) {
      message.success('模型创建成功');
    }
  }
  showEditor.value = false;
  editingModel.value = null;
};

const handleDeleteModel = async (id: string) => {
  if (confirm('确定要删除这个模型信息吗？')) {
    await store.deleteModel(id);
    message.success('模型删除成功');
  }
};

onMounted(() => {
  store.fetchModels();
});
</script>

<template>
  <div class="h-full flex flex-col">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <div class="flex items-center gap-4 mb-4">
        <div class="flex items-center gap-2">
          <NButton
            v-for="tab in typeTabs"
            :key="tab.key"
            :type="activeType === tab.key ? 'primary' : 'default'"
            size="small"
            @click="handleTypeChange(tab.key)"
          >
            <template #icon>
              <NIcon :component="tab.icon" />
            </template>
            {{ tab.label }}
          </NButton>
        </div>
      </div>

      <div class="flex items-center gap-4">
        <div class="flex-1">
          <NInput
            v-model:value="searchQuery"
            placeholder="搜索模型名称、简介、标签..."
            clearable
            @update:value="handleSearch"
            style="min-width: 200px"
          >
            <template #prefix>
              <NIcon :component="Search" />
            </template>
          </NInput>
        </div>

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

        <NButton type="primary" @click="handleCreateModel">
          <template #icon>
            <NIcon :component="Plus" />
          </template>
          添加模型
        </NButton>
      </div>
    </div>

    <div class="flex-1 overflow-auto p-4">
      <NSpin :show="store.isLoading">
        <div v-if="store.filteredModels.length === 0" class="h-full flex items-center justify-center">
          <NEmpty description="暂无模型信息">
            <template #extra>
              <NButton size="small" @click="handleCreateModel">
                添加第一个模型
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
          <ModelCard
            v-for="model in store.filteredModels"
            :key="model.id"
            :model="model"
            :view-mode="viewMode"
            @edit="handleEditModel"
            @delete="handleDeleteModel"
          />
        </div>
      </NSpin>
    </div>

    <ModelEditor
      v-if="showEditor"
      :model="editingModel"
      @close="handleEditorClose"
      @save="handleEditorSave"
    />
  </div>
</template>
