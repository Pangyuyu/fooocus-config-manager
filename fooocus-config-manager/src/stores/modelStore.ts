import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { ModelInfo, ModelType, ModelFilterOptions, ModelUsageInfo } from '../types';
import { createEmptyModelInfo } from '../types';

interface ModelState {
  models: ModelInfo[];
  currentModel: ModelInfo | null;
  filter: ModelFilterOptions;
  isLoading: boolean;
  error: string | null;
}

export const useModelStore = defineStore('model', {
  state: (): ModelState => ({
    models: [],
    currentModel: null,
    filter: {
      search: '',
      type: '',
      tags: [],
      sortBy: 'updatedAt',
      sortOrder: 'desc',
    },
    isLoading: false,
    error: null,
  }),

  getters: {
    filteredModels: (state) => {
      let result = [...state.models];

      if (state.filter.search) {
        const search = state.filter.search.toLowerCase();
        result = result.filter(m => 
          m.name.toLowerCase().includes(search) ||
          m.description.toLowerCase().includes(search) ||
          m.scope.some(s => s.toLowerCase().includes(search)) ||
          m.tags.some(t => t.toLowerCase().includes(search))
        );
      }

      if (state.filter.type) {
        result = result.filter(m => m.type === state.filter.type);
      }

      if (state.filter.tags.length > 0) {
        result = result.filter(m => 
          state.filter.tags.some(tag => m.tags.includes(tag) || m.scope.includes(tag))
        );
      }

      result.sort((a, b) => {
        let comparison = 0;
        switch (state.filter.sortBy) {
          case 'name':
            comparison = a.name.localeCompare(b.name);
            break;
          case 'createdAt':
            comparison = new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
            break;
          case 'updatedAt':
            comparison = new Date(a.updatedAt).getTime() - new Date(b.updatedAt).getTime();
            break;
        }
        return state.filter.sortOrder === 'desc' ? -comparison : comparison;
      });

      return result;
    },

    checkpoints: (state) => state.models.filter(m => m.type === 'Checkpoint'),
    loras: (state) => state.models.filter(m => m.type === 'LoRA'),
    refiners: (state) => state.models.filter(m => m.type === 'Refiner'),
    embeddings: (state) => state.models.filter(m => m.type === 'Embedding'),

    modelsByType: (state) => (type: ModelType) => state.models.filter(m => m.type === type),

    getModelById: (state) => (id: string) => state.models.find(m => m.id === id),

    allTags: (state) => {
      const tags = new Set<string>();
      state.models.forEach(m => {
        m.tags.forEach(t => tags.add(t));
        m.scope.forEach(s => tags.add(s));
      });
      return Array.from(tags).sort();
    },

    scopeTags: (state) => {
      const scopes = new Set<string>();
      state.models.forEach(m => {
        m.scope.forEach(s => scopes.add(s));
      });
      return Array.from(scopes).sort();
    },
  },

  actions: {
    async fetchModels() {
      this.isLoading = true;
      this.error = null;
      try {
        const models = await invoke<ModelInfo[]>('get_all_models');
        this.models = models;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to fetch models:', e);
      } finally {
        this.isLoading = false;
      }
    },

    async fetchModelsByType(type: ModelType) {
      this.isLoading = true;
      this.error = null;
      try {
        const models = await invoke<ModelInfo[]>('get_models_by_type', { modelType: type });
        return models;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to fetch models by type:', e);
        return [];
      } finally {
        this.isLoading = false;
      }
    },

    async fetchModelById(id: string) {
      this.isLoading = true;
      this.error = null;
      try {
        const model = await invoke<ModelInfo | null>('get_model_by_id', { id });
        this.currentModel = model;
        return model;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to fetch model:', e);
        return null;
      } finally {
        this.isLoading = false;
      }
    },

    async createModel(model?: Partial<ModelInfo>) {
      this.isLoading = true;
      this.error = null;
      try {
        const newModel = {
          ...createEmptyModelInfo(),
          ...model,
        };
        const created = await invoke<ModelInfo>('create_model', { model: newModel });
        this.models.unshift(created);
        return created;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to create model:', e);
        return null;
      } finally {
        this.isLoading = false;
      }
    },

    async updateModel(model: ModelInfo) {
      this.isLoading = true;
      this.error = null;
      try {
        const updated = await invoke<ModelInfo>('update_model', { model });
        const index = this.models.findIndex(m => m.id === model.id);
        if (index !== -1) {
          this.models[index] = updated;
        }
        if (this.currentModel?.id === model.id) {
          this.currentModel = updated;
        }
        return updated;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to update model:', e);
        return null;
      } finally {
        this.isLoading = false;
      }
    },

    async deleteModel(id: string) {
      this.isLoading = true;
      this.error = null;
      try {
        await invoke('delete_model', { id });
        this.models = this.models.filter(m => m.id !== id);
        if (this.currentModel?.id === id) {
          this.currentModel = null;
        }
      } catch (e) {
        this.error = String(e);
        console.error('Failed to delete model:', e);
      } finally {
        this.isLoading = false;
      }
    },

    async searchModels(query: string) {
      this.isLoading = true;
      this.error = null;
      try {
        const models = await invoke<ModelInfo[]>('search_models', { query });
        return models;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to search models:', e);
        return [];
      } finally {
        this.isLoading = false;
      }
    },

    async checkModelUsage(modelId: string): Promise<ModelUsageInfo | null> {
      try {
        const usageInfo = await invoke<ModelUsageInfo>('check_model_usage', { modelId });
        return usageInfo;
      } catch (e) {
        console.error('Failed to check model usage:', e);
        return null;
      }
    },

    setFilter(filter: Partial<ModelFilterOptions>) {
      this.filter = { ...this.filter, ...filter };
    },

    setCurrentModel(model: ModelInfo | null) {
      this.currentModel = model;
    },

    clearError() {
      this.error = null;
    },
  },
});
