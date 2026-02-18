<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import {
  NModal, NForm, NFormItem, NInput, NButton, NSpace, NIcon,
  NDynamicTags, NSelect, NDivider
} from 'naive-ui';
import { X, Save } from 'lucide-vue-next';
import type { ModelInfo } from '../types';
import { createEmptyModelInfo, MODEL_TYPES, DEFAULT_SCOPE_TAGS } from '../types';

const props = defineProps<{
  model: ModelInfo | null;
}>();

const emit = defineEmits<{
  close: [];
  save: [model: ModelInfo];
}>();

const formData = ref<ModelInfo>(createEmptyModelInfo());

const isNewModel = computed(() => !props.model);

watch(() => props.model, (newModel) => {
  if (newModel) {
    formData.value = JSON.parse(JSON.stringify(newModel));
  } else {
    formData.value = createEmptyModelInfo();
  }
}, { immediate: true });

const typeOptions = MODEL_TYPES.map(t => ({ label: t, value: t }));
const scopeOptions = DEFAULT_SCOPE_TAGS.map(s => ({ label: s, value: s }));

const handleSave = () => {
  formData.value.updatedAt = new Date().toISOString();
  if (isNewModel.value) {
    formData.value.createdAt = formData.value.updatedAt;
  }
  emit('save', formData.value);
};

const handleClose = () => {
  emit('close');
};
</script>

<template>
  <NModal
    :show="true"
    :mask-closable="false"
    :close-on-esc="true"
    preset="card"
    :style="{ width: '600px', maxHeight: '90vh' }"
    :title="isNewModel ? '添加模型' : '编辑模型'"
    :bordered="false"
    @esc="handleClose"
  >
    <template #header-extra>
      <NButton quaternary circle @click="handleClose">
        <template #icon>
          <NIcon :component="X" />
        </template>
      </NButton>
    </template>

    <div class="max-h-[70vh] overflow-auto">
      <NForm label-placement="left" label-width="80">
        <div class="py-4 space-y-4">
          <NFormItem label="模型名称" required>
            <NInput v-model:value="formData.name" placeholder="输入模型显示名称" />
          </NFormItem>

          <NFormItem label="模型类型" required>
            <NSelect
              v-model:value="formData.type"
              :options="typeOptions"
              placeholder="选择模型类型"
            />
          </NFormItem>

          <NFormItem label="文件名">
            <NInput v-model:value="formData.fileName" placeholder="模型文件名（如 model.safetensors）" />
          </NFormItem>

          <NFormItem label="模型路径">
            <NInput v-model:value="formData.path" placeholder="本地路径或下载链接" />
          </NFormItem>

          <NDivider>详细信息</NDivider>

          <NFormItem label="模型简介">
            <NInput
              v-model:value="formData.description"
              type="textarea"
              placeholder="输入模型简介"
              :rows="3"
            />
          </NFormItem>

          <NFormItem label="适用范围">
            <NSelect
              v-model:value="formData.scope"
              :options="scopeOptions"
              multiple
              filterable
              tag
              placeholder="选择或输入适用范围标签"
            />
          </NFormItem>

          <NFormItem label="自定义标签">
            <NDynamicTags v-model:value="formData.tags" />
          </NFormItem>
        </div>
      </NForm>
    </div>

    <template #footer>
      <NSpace justify="end">
        <NButton @click="handleClose">取消</NButton>
        <NButton type="primary" @click="handleSave">
          <template #icon>
            <NIcon :component="Save" />
          </template>
          保存
        </NButton>
      </NSpace>
    </template>
  </NModal>
</template>
