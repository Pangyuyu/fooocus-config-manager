export interface LoRA {
  name: string;
  modelName: string;
  weight: number;
}

export interface ModelConfig {
  baseModel: string;
  refinerModel: string;
  refinerSwitch: number;
  loras: LoRA[];
}

export type PerformanceMode = 'Speed' | 'Quality' | 'Lightning';

export interface SamplingConfig {
  cfgScale: number;
  sampleSharpness: number;
  sampler: string;
  scheduler: string;
  performance: PerformanceMode;
  steps: number;
}

export interface PromptConfig {
  positive: string;
  negative: string;
  styles: string[];
}

export interface ImageConfig {
  aspectRatio: string;
  imageCount: number;
}

export interface ResourceDownloads {
  checkpointDownloads?: Record<string, string>;
  loraDownloads?: Record<string, string>;
  embeddingDownloads?: Record<string, string>;
}

export interface PresetConfig {
  id: string;
  name: string;
  description: string;
  tags: string[];
  isFavorite: boolean;
  useCount: number;
  createdAt: string;
  updatedAt: string;
  model: ModelConfig;
  sampling: SamplingConfig;
  prompt: PromptConfig;
  image: ImageConfig;
  resources?: ResourceDownloads;
}

export interface Tag {
  id: string;
  name: string;
  color: string;
  count: number;
}

export interface FilterOptions {
  search: string;
  tags: string[];
  isFavorite: boolean | null;
  baseModel: string;
  sortBy: 'name' | 'updatedAt' | 'createdAt' | 'useCount';
  sortOrder: 'asc' | 'desc';
}

export const DEFAULT_SAMPLERS = [
  'dpmpp_2m_sde_gpu',
  'dpmpp_2m_sde',
  'dpmpp_2m',
  'euler',
  'euler_a',
  'heun',
  'dpm_2',
  'dpm_2_a',
  'lms',
  'dpm_fast',
  'dpm_adaptive',
  'ddim',
] as const;

export const DEFAULT_SCHEDULERS = [
  'karras',
  'normal',
  'exponential',
  'sgm_uniform',
  'simple',
  'ddim_uniform',
] as const;

export const DEFAULT_ASPECT_RATIOS = [
  '704*1408',
  '704*1344',
  '768*1344',
  '768*1280',
  '832*1216',
  '832*1152',
  '896*1152',
  '896*1088',
  '960*1088',
  '960*1024',
  '1024*1024',
  '1024*960',
  '1088*960',
  '1088*896',
  '1152*896',
  '1152*832',
  '1216*832',
  '1280*768',
  '1344*768',
  '1344*704',
  '1408*704',
  '1472*704',
  '1536*640',
  '1600*640',
  '1664*576',
  '1728*576',
] as const;

export function createEmptyPresetConfig(): PresetConfig {
  const now = new Date().toISOString();
  return {
    id: crypto.randomUUID(),
    name: '',
    description: '',
    tags: [],
    isFavorite: false,
    useCount: 0,
    createdAt: now,
    updatedAt: now,
    model: {
      baseModel: '',
      refinerModel: '',
      refinerSwitch: 0.5,
      loras: [],
    },
    sampling: {
      cfgScale: 7.0,
      sampleSharpness: 2.0,
      sampler: 'dpmpp_2m_sde_gpu',
      scheduler: 'karras',
      performance: 'Speed',
      steps: 30,
    },
    prompt: {
      positive: '',
      negative: '',
      styles: [],
    },
    image: {
      aspectRatio: '1152*896',
      imageCount: 4,
    },
  };
}
