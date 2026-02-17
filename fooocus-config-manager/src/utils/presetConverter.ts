import type { PresetConfig, LoRA, PerformanceMode } from '../types';

interface FooocusPreset {
  default_model: string;
  default_refiner_model: string;
  default_refiner_switch: number;
  default_loras: Array<[string, string, number]>;
  default_cfg_scale: number;
  default_sample_sharpness: number;
  default_sampler: string;
  default_scheduler: string;
  default_performance: string;
  default_steps: number;
  default_prompt_negative: string;
  default_prompt_positive: string;
  default_styles: string[];
  default_aspect_ratio: string;
  default_overwrite_step: number;
  default_overwrite_switch: number;
  default_overwrite_width: number;
  default_overwrite_height: number;
  default_cfg_tsnr: number;
  default_negative_prompt: string;
  default_positive_prompt: string;
  checkpoint_downloads?: Record<string, string>;
  lora_downloads?: Record<string, string>;
  embedding_downloads?: Record<string, string>;
}

export function exportToFooocusPreset(preset: PresetConfig): FooocusPreset {
  return {
    default_model: preset.model.baseModel,
    default_refiner_model: preset.model.refinerModel,
    default_refiner_switch: preset.model.refinerSwitch,
    default_loras: preset.model.loras.map(lora => [
      lora.name,
      lora.modelName,
      lora.weight,
    ] as [string, string, number]),
    default_cfg_scale: preset.sampling.cfgScale,
    default_sample_sharpness: preset.sampling.sampleSharpness,
    default_sampler: preset.sampling.sampler,
    default_scheduler: preset.sampling.scheduler,
    default_performance: preset.sampling.performance,
    default_steps: preset.sampling.steps,
    default_prompt_negative: preset.prompt.negative,
    default_prompt_positive: preset.prompt.positive,
    default_styles: preset.prompt.styles,
    default_aspect_ratio: preset.image.aspectRatio,
    default_overwrite_step: -1,
    default_overwrite_switch: -1,
    default_overwrite_width: -1,
    default_overwrite_height: -1,
    default_cfg_tsnr: 7,
    default_negative_prompt: preset.prompt.negative,
    default_positive_prompt: preset.prompt.positive,
    checkpoint_downloads: preset.resources?.checkpointDownloads,
    lora_downloads: preset.resources?.loraDownloads,
    embedding_downloads: preset.resources?.embeddingDownloads,
  };
}

export function importFromFooocusPreset(
  fooocusPreset: FooocusPreset,
  name: string = ''
): PresetConfig {
  const now = new Date().toISOString();
  
  const loras: LoRA[] = (fooocusPreset.default_loras || []).map(([name, modelName, weight]) => ({
    name,
    modelName,
    weight,
  }));

  return {
    id: crypto.randomUUID(),
    name: name || fooocusPreset.default_model || '导入的配置',
    description: `从 Fooocus 预设导入 - ${fooocusPreset.default_model}`,
    tags: [],
    isFavorite: false,
    useCount: 0,
    createdAt: now,
    updatedAt: now,
    model: {
      baseModel: fooocusPreset.default_model || '',
      refinerModel: fooocusPreset.default_refiner_model || '',
      refinerSwitch: fooocusPreset.default_refiner_switch || 0.5,
      loras,
    },
    sampling: {
      cfgScale: fooocusPreset.default_cfg_scale || 7.0,
      sampleSharpness: fooocusPreset.default_sample_sharpness || 2.0,
      sampler: fooocusPreset.default_sampler || 'dpmpp_2m_sde_gpu',
      scheduler: fooocusPreset.default_scheduler || 'karras',
      performance: (fooocusPreset.default_performance || 'Speed') as PerformanceMode,
      steps: fooocusPreset.default_steps || 30,
    },
    prompt: {
      positive: fooocusPreset.default_prompt_positive || fooocusPreset.default_positive_prompt || '',
      negative: fooocusPreset.default_prompt_negative || fooocusPreset.default_negative_prompt || '',
      styles: fooocusPreset.default_styles || [],
    },
    image: {
      aspectRatio: fooocusPreset.default_aspect_ratio || '1152*896',
      imageCount: 4,
    },
    resources: {
      checkpointDownloads: fooocusPreset.checkpoint_downloads,
      loraDownloads: fooocusPreset.lora_downloads,
      embeddingDownloads: fooocusPreset.embedding_downloads,
    },
  };
}

export async function downloadPresetAsJson(preset: PresetConfig): Promise<void> {
  const fooocusPreset = exportToFooocusPreset(preset);
  const json = JSON.stringify(fooocusPreset, null, 2);
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  
  const a = document.createElement('a');
  a.href = url;
  a.download = `${preset.name || 'preset'}.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

export function parseFooocusPresetJson(jsonString: string): PresetConfig {
  const fooocusPreset: FooocusPreset = JSON.parse(jsonString);
  return importFromFooocusPreset(fooocusPreset);
}
