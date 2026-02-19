<script setup lang="ts">
import { computed } from 'vue';
import { NCard, NButton, NIcon, NTag, NPopconfirm } from 'naive-ui';
import { Star, Edit, Trash2, Copy, Download } from 'lucide-vue-next';
import type { PresetConfig } from '../types';
import { useModelStore } from '../stores/modelStore';

const props = defineProps<{
  preset: PresetConfig;
  viewMode: 'grid' | 'list';
}>();

const emit = defineEmits<{
  edit: [preset: PresetConfig];
  delete: [id: string];
  toggleFavorite: [id: string];
  export: [preset: PresetConfig];
}>();

const modelStore = useModelStore();

const formattedDate = computed(() => {
  return new Date(props.preset.updatedAt).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
});

const truncatedDescription = computed(() => {
  if (props.preset.description.length > 100) {
    return props.preset.description.slice(0, 100) + '...';
  }
  return props.preset.description || '暂无描述';
});

const displayBaseModel = computed(() => {
  if (props.preset.model.baseModelId) {
    const model = modelStore.getModelById(props.preset.model.baseModelId);
    if (model) {
      return model.fileName || model.name;
    }
  }
  return props.preset.model.baseModel;
});

const handleCopy = async () => {
  const configText = JSON.stringify(props.preset, null, 2);
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
          <h3 class="font-medium text-gray-900 dark:text-gray-100 truncate flex-1">
            {{ preset.name || '未命名配置' }}
          </h3>
          <NButton
            quaternary
            circle
            size="tiny"
            @click.stop="emit('toggleFavorite', preset.id)"
          >
            <template #icon>
              <NIcon :component="Star" :class="preset.isFavorite ? 'text-yellow-500 fill-yellow-500' : ''" />
            </template>
          </NButton>
        </div>

        <p class="text-sm text-gray-500 dark:text-gray-400 mb-3 line-clamp-2">
          {{ truncatedDescription }}
        </p>

        <div class="flex flex-wrap gap-1 mb-3">
          <NTag
            v-if="displayBaseModel"
            size="small"
            type="info"
            :bordered="false"
          >
            {{ displayBaseModel }}
          </NTag>
          <NTag
            v-for="tag in preset.tags.slice(0, 3)"
            :key="tag"
            size="small"
            :bordered="false"
          >
            {{ tag }}
          </NTag>
          <NTag v-if="preset.tags.length > 3" size="small" :bordered="false">
            +{{ preset.tags.length - 3 }}
          </NTag>
        </div>

        <div class="flex items-center gap-2 text-xs text-gray-400">
          <span>{{ formattedDate }}</span>
          <span>·</span>
          <span>使用 {{ preset.useCount }} 次</span>
        </div>
      </div>

      <div :class="[viewMode === 'list' ? 'flex items-center gap-1 ml-4' : 'flex items-center gap-1 mt-3 pt-3 border-t border-gray-100 dark:border-gray-700']">
        <NButton quaternary size="small" @click.stop="emit('edit', preset)">
          <template #icon>
            <NIcon :component="Edit" />
          </template>
        </NButton>
        <NButton quaternary size="small" @click.stop="handleCopy">
          <template #icon>
            <NIcon :component="Copy" />
          </template>
        </NButton>
        <NButton quaternary size="small" @click.stop="emit('export', preset)">
          <template #icon>
            <NIcon :component="Download" />
          </template>
        </NButton>
        <NPopconfirm @positive-click="emit('delete', preset.id)">
          <template #trigger>
            <NButton quaternary size="small" type="error" @click.stop>
              <template #icon>
                <NIcon :component="Trash2" />
              </template>
            </NButton>
          </template>
          确定要删除这个配置吗？
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
