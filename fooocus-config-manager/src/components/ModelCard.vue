<script setup lang="ts">
import { computed } from 'vue';
import { NCard, NButton, NIcon, NTag, NPopconfirm } from 'naive-ui';
import { Edit, Trash2, Copy, Box, Layers, Sparkles, FileCode } from 'lucide-vue-next';
import type { ModelInfo, ModelType } from '../types';

const props = defineProps<{
  model: ModelInfo;
  viewMode: 'grid' | 'list';
}>();

const emit = defineEmits<{
  edit: [model: ModelInfo];
  delete: [id: string];
}>();

const typeIcons: Record<ModelType, typeof Box> = {
  Checkpoint: Box,
  LoRA: Layers,
  Refiner: Sparkles,
  Embedding: FileCode,
};

const typeColors: Record<ModelType, string> = {
  Checkpoint: 'info',
  LoRA: 'success',
  Refiner: 'warning',
  Embedding: 'default',
};

const formattedDate = computed(() => {
  return new Date(props.model.updatedAt).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
});

const truncatedDescription = computed(() => {
  if (props.model.description.length > 100) {
    return props.model.description.slice(0, 100) + '...';
  }
  return props.model.description || '暂无描述';
});

const handleCopy = async () => {
  const configText = JSON.stringify(props.model, null, 2);
  await navigator.clipboard.writeText(configText);
};
</script>

<template>
  <NCard
    :class="[
      'cursor-pointer transition-all hover:shadow-md',
      viewMode === 'list' ? 'flex items-center' : ''
    ]"
    size="small"
    hoverable
  >
    <div :class="[viewMode === 'list' ? 'flex items-center gap-4 w-full' : '']">
      <div :class="[viewMode === 'list' ? 'flex-1 min-w-0' : '']">
        <div class="flex items-start justify-between mb-2">
          <div class="flex items-center gap-2 flex-1 min-w-0">
            <NIcon :component="typeIcons[model.type]" :size="18" class="text-indigo-500 flex-shrink-0" />
            <h3 class="font-medium text-gray-900 dark:text-gray-100 truncate">
              {{ model.name || '未命名模型' }}
            </h3>
          </div>
          <NTag :type="typeColors[model.type] as 'info' | 'success' | 'warning' | 'default'" size="small" :bordered="false">
            {{ model.type }}
          </NTag>
        </div>

        <p class="text-sm text-gray-500 dark:text-gray-400 mb-3 line-clamp-2">
          {{ truncatedDescription }}
        </p>

        <div v-if="model.fileName" class="text-xs text-gray-400 mb-2 truncate">
          文件: {{ model.fileName }}
        </div>

        <div class="flex flex-wrap gap-1 mb-3">
          <NTag
            v-for="scope in model.scope.slice(0, 2)"
            :key="scope"
            size="small"
            type="info"
            :bordered="false"
          >
            {{ scope }}
          </NTag>
          <NTag
            v-for="tag in model.tags.slice(0, 2)"
            :key="tag"
            size="small"
            :bordered="false"
          >
            {{ tag }}
          </NTag>
          <NTag v-if="model.scope.length + model.tags.length > 4" size="small" :bordered="false">
            +{{ model.scope.length + model.tags.length - 4 }}
          </NTag>
        </div>

        <div class="flex items-center gap-2 text-xs text-gray-400">
          <span>{{ formattedDate }}</span>
        </div>
      </div>

      <div :class="[viewMode === 'list' ? 'flex items-center gap-1 ml-4' : 'flex items-center gap-1 mt-3 pt-3 border-t border-gray-100 dark:border-gray-700']">
        <NButton quaternary size="small" @click.stop="emit('edit', model)">
          <template #icon>
            <NIcon :component="Edit" />
          </template>
        </NButton>
        <NButton quaternary size="small" @click.stop="handleCopy">
          <template #icon>
            <NIcon :component="Copy" />
          </template>
        </NButton>
        <NPopconfirm @positive-click="emit('delete', model.id)">
          <template #trigger>
            <NButton quaternary size="small" type="error" @click.stop>
              <template #icon>
                <NIcon :component="Trash2" />
              </template>
            </NButton>
          </template>
          确定要删除这个模型信息吗？
        </NPopconfirm>
      </div>
    </div>
  </NCard>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
