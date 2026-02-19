<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import {
  NModal, NForm, NFormItem, NInput, NInputNumber, NSelect, NButton,
  NSpace, NIcon, NDynamicTags, NDivider, NTabs, NTabPane
} from 'naive-ui';
import { X, Plus, Save } from 'lucide-vue-next';
import type { PresetConfig, ModelInfo } from '../types';
import { createEmptyPresetConfig, DEFAULT_SAMPLERS, DEFAULT_SCHEDULERS, DEFAULT_ASPECT_RATIOS } from '../types';
import { useModelStore } from '../stores/modelStore';

const props = defineProps<{
  preset: PresetConfig | null;
}>();

const emit = defineEmits<{
  close: [];
  save: [preset: PresetConfig];
}>();

const modelStore = useModelStore();
const formData = ref<PresetConfig>(createEmptyPresetConfig());
const activeTab = ref('basic');

const isNewPreset = computed(() => !props.preset);

watch(() => props.preset, (newPreset) => {
  if (newPreset) {
    formData.value = JSON.parse(JSON.stringify(newPreset));
  } else {
    formData.value = createEmptyPresetConfig();
  }
}, { immediate: true });

const samplerOptions = DEFAULT_SAMPLERS.map(s => ({ label: s, value: s }));
const schedulerOptions = DEFAULT_SCHEDULERS.map(s => ({ label: s, value: s }));
const aspectRatioOptions = DEFAULT_ASPECT_RATIOS.map(r => ({ label: r, value: r }));
const performanceOptions = [
  { label: '速度优先', value: 'Speed' },
  { label: '质量优先', value: 'Quality' },
  { label: 'Lightning', value: 'Lightning' },
];

const checkpointOptions = computed(() => {
  const models = modelStore.checkpoints;
  const options = models.map((m: ModelInfo) => ({
    label: m.name,
    value: m.id,
    fileName: m.fileName,
  }));
  return options;
});

const refinerOptions = computed(() => {
  const models = modelStore.refiners;
  const options = models.map((m: ModelInfo) => ({
    label: m.name,
    value: m.id,
    fileName: m.fileName,
  }));
  const emptyOption = { label: '无精修模型', value: '' };
  return [emptyOption, ...options];
});

const loraOptions = computed(() => {
  const models = modelStore.loras;
  return models.map((m: ModelInfo) => ({
    label: m.name,
    value: m.id,
    fileName: m.fileName,
  }));
});

const handleBaseModelSelect = (modelId: string | null) => {
  if (modelId) {
    const model = modelStore.getModelById(modelId);
    if (model) {
      formData.value.model.baseModelId = modelId;
      formData.value.model.baseModel = model.fileName;
    }
  } else {
    formData.value.model.baseModelId = undefined;
  }
};

const handleRefinerModelSelect = (modelId: string | null) => {
  if (modelId) {
    const model = modelStore.getModelById(modelId);
    if (model) {
      formData.value.model.refinerModelId = modelId;
      formData.value.model.refinerModel = model.fileName;
    }
  } else {
    formData.value.model.refinerModelId = undefined;
    formData.value.model.refinerModel = '';
  }
};

const handleLoraModelSelect = (index: number, modelId: string | null) => {
  if (modelId) {
    const model = modelStore.getModelById(modelId);
    if (model) {
      formData.value.model.loras[index].modelId = modelId;
      formData.value.model.loras[index].modelName = model.fileName;
      if (!formData.value.model.loras[index].name) {
        formData.value.model.loras[index].name = model.name;
      }
    }
  } else {
    formData.value.model.loras[index].modelId = undefined;
  }
};

const handleAddLora = () => {
  formData.value.model.loras.push({
    name: '',
    modelName: '',
    weight: 1.0,
  });
};

const handleRemoveLora = (index: number) => {
  formData.value.model.loras.splice(index, 1);
};

const handleSave = () => {
  formData.value.updatedAt = new Date().toISOString();
  if (isNewPreset.value) {
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
    :closable="false"
    preset="card"
    :style="{ width: '800px', maxHeight: '90vh' }"
    :title="isNewPreset ? '新建配置' : '编辑配置'"
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
      <NForm label-placement="left" label-width="100">
        <NTabs v-model:value="activeTab" type="line">
          <NTabPane name="basic" tab="基本信息">
            <div class="py-4 space-y-4">
              <NFormItem label="配置名称" required>
                <NInput v-model:value="formData.name" placeholder="输入配置名称" />
              </NFormItem>

              <NFormItem label="描述">
                <NInput
                  v-model:value="formData.description"
                  type="textarea"
                  placeholder="输入配置描述"
                  :rows="3"
                />
              </NFormItem>

              <NFormItem label="标签">
                <NDynamicTags v-model:value="formData.tags" />
              </NFormItem>
            </div>
          </NTabPane>

          <NTabPane name="model" tab="模型配置">
            <div class="py-4 space-y-4">
              <NFormItem label="基础模型">
                <div class="flex gap-2 w-full">
                  <NSelect
                    :value="formData.model.baseModelId"
                    :options="checkpointOptions"
                    placeholder="从模型库选择"
                    filterable
                    clearable
                    class="flex-1"
                    @update:value="handleBaseModelSelect"
                  />
                  <NInput
                    v-model:value="formData.model.baseModel"
                    placeholder="或手动输入模型名称"
                    class="flex-1"
                  />
                </div>
              </NFormItem>

              <NFormItem label="精修模型">
                <div class="flex gap-2 w-full">
                  <NSelect
                    :value="formData.model.refinerModelId"
                    :options="refinerOptions"
                    placeholder="从模型库选择"
                    filterable
                    clearable
                    class="flex-1"
                    @update:value="handleRefinerModelSelect"
                  />
                  <NInput
                    v-model:value="formData.model.refinerModel"
                    placeholder="或手动输入模型名称"
                    class="flex-1"
                  />
                </div>
              </NFormItem>

              <NFormItem label="精修切换点">
                <NInputNumber
                  v-model:value="formData.model.refinerSwitch"
                  :min="0"
                  :max="1"
                  :step="0.1"
                />
              </NFormItem>

              <NDivider>LoRA 配置</NDivider>

              <div v-for="(lora, index) in formData.model.loras" :key="index" class="flex items-start gap-2 mb-4 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div class="flex-1 space-y-2">
                  <NInput v-model:value="lora.name" placeholder="LoRA 名称" size="small" />
                  <div class="flex gap-2">
                    <NSelect
                      :value="lora.modelId"
                      :options="loraOptions"
                      placeholder="从模型库选择"
                      filterable
                      clearable
                      class="flex-1"
                      size="small"
                      @update:value="(val: string | null) => handleLoraModelSelect(index, val)"
                    />
                    <NInput
                      v-model:value="lora.modelName"
                      placeholder="或手动输入文件名"
                      class="flex-1"
                      size="small"
                    />
                  </div>
                  <NInputNumber
                    v-model:value="lora.weight"
                    placeholder="权重"
                    :min="-1"
                    :max="2"
                    :step="0.1"
                    size="small"
                  />
                </div>
                <NButton quaternary type="error" size="small" @click="handleRemoveLora(index)">
                  <template #icon>
                    <NIcon :component="X" />
                  </template>
                </NButton>
              </div>

              <NButton dashed block @click="handleAddLora">
                <template #icon>
                  <NIcon :component="Plus" />
                </template>
                添加 LoRA
              </NButton>
            </div>
          </NTabPane>

          <NTabPane name="sampling" tab="采样参数">
            <div class="py-4 space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <NFormItem label="CFG Scale">
                  <NInputNumber
                    v-model:value="formData.sampling.cfgScale"
                    :min="1"
                    :max="30"
                    :step="0.5"
                  />
                </NFormItem>

                <NFormItem label="采样锐度">
                  <NInputNumber
                    v-model:value="formData.sampling.sampleSharpness"
                    :min="0"
                    :max="10"
                    :step="0.5"
                  />
                </NFormItem>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <NFormItem label="采样器">
                  <NSelect
                    v-model:value="formData.sampling.sampler"
                    :options="samplerOptions"
                  />
                </NFormItem>

                <NFormItem label="调度器">
                  <NSelect
                    v-model:value="formData.sampling.scheduler"
                    :options="schedulerOptions"
                  />
                </NFormItem>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <NFormItem label="性能模式">
                  <NSelect
                    v-model:value="formData.sampling.performance"
                    :options="performanceOptions"
                  />
                </NFormItem>

                <NFormItem label="步数">
                  <NInputNumber
                    v-model:value="formData.sampling.steps"
                    :min="1"
                    :max="200"
                  />
                </NFormItem>
              </div>
            </div>
          </NTabPane>

          <NTabPane name="prompt" tab="提示词">
            <div class="py-4 space-y-4">
              <NFormItem label="正向提示词">
                <NInput
                  v-model:value="formData.prompt.positive"
                  type="textarea"
                  placeholder="输入正向提示词"
                  :rows="5"
                />
              </NFormItem>

              <NFormItem label="反向提示词">
                <NInput
                  v-model:value="formData.prompt.negative"
                  type="textarea"
                  placeholder="输入反向提示词"
                  :rows="3"
                />
              </NFormItem>

              <NFormItem label="风格">
                <NDynamicTags v-model:value="formData.prompt.styles" />
              </NFormItem>
            </div>
          </NTabPane>

          <NTabPane name="image" tab="图像参数">
            <div class="py-4 space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <NFormItem label="宽高比">
                  <NSelect
                    v-model:value="formData.image.aspectRatio"
                    :options="aspectRatioOptions"
                    filterable
                  />
                </NFormItem>

                <NFormItem label="图像数量">
                  <NInputNumber
                    v-model:value="formData.image.imageCount"
                    :min="1"
                    :max="16"
                  />
                </NFormItem>
              </div>
            </div>
          </NTabPane>
        </NTabs>
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
