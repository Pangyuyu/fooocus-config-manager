import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { PresetConfig, Tag, FilterOptions } from '../types';
import { createEmptyPresetConfig } from '../types';
import { useModelStore } from './modelStore';

interface PresetState {
  presets: PresetConfig[];
  tags: Tag[];
  currentPreset: PresetConfig | null;
  filter: FilterOptions;
  isLoading: boolean;
  error: string | null;
}

export const usePresetStore = defineStore('preset', {
  state: (): PresetState => ({
    presets: [],
    tags: [],
    currentPreset: null,
    filter: {
      search: '',
      tags: [],
      isFavorite: null,
      baseModel: '',
      sortBy: 'updatedAt',
      sortOrder: 'desc',
    },
    isLoading: false,
    error: null,
  }),

  getters: {
    filteredPresets: (state) => {
      const modelStore = useModelStore();
      let result = [...state.presets];

      if (state.filter.search) {
        const search = state.filter.search.toLowerCase();
        result = result.filter(p => 
          p.name.toLowerCase().includes(search) ||
          p.description.toLowerCase().includes(search) ||
          p.tags.some(t => t.toLowerCase().includes(search))
        );
      }

      if (state.filter.tags.length > 0) {
        result = result.filter(p => 
          state.filter.tags.some(tag => p.tags.includes(tag))
        );
      }

      if (state.filter.isFavorite !== null) {
        result = result.filter(p => p.isFavorite === state.filter.isFavorite);
      }

      if (state.filter.baseModel) {
        result = result.filter(p => {
          if (p.model.baseModelId) {
            const model = modelStore.getModelById(p.model.baseModelId);
            if (model) {
              const modelDisplay = model.fileName || model.name;
              return modelDisplay.toLowerCase().includes(state.filter.baseModel.toLowerCase());
            }
          }
          return p.model.baseModel.toLowerCase().includes(state.filter.baseModel.toLowerCase());
        });
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
          case 'useCount':
            comparison = a.useCount - b.useCount;
            break;
        }
        return state.filter.sortOrder === 'desc' ? -comparison : comparison;
      });

      return result;
    },

    favoritePresets: (state) => state.presets.filter(p => p.isFavorite),

    baseModels: (state) => {
      const modelStore = useModelStore();
      const models = new Set<string>();
      state.presets.forEach(p => {
        if (p.model.baseModelId) {
          const model = modelStore.getModelById(p.model.baseModelId);
          if (model) {
            models.add(model.fileName || model.name);
            return;
          }
        }
        if (p.model.baseModel) {
          models.add(p.model.baseModel);
        }
      });
      return Array.from(models).sort();
    },

    getPresetById: (state) => (id: string) => state.presets.find(p => p.id === id),
  },

  actions: {
    async fetchPresets() {
      this.isLoading = true;
      this.error = null;
      try {
        const presets = await invoke<PresetConfig[]>('get_all_presets');
        this.presets = presets;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to fetch presets:', e);
      } finally {
        this.isLoading = false;
      }
    },

    async fetchPresetById(id: string) {
      this.isLoading = true;
      this.error = null;
      try {
        const preset = await invoke<PresetConfig | null>('get_preset_by_id', { id });
        this.currentPreset = preset;
        return preset;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to fetch preset:', e);
        return null;
      } finally {
        this.isLoading = false;
      }
    },

    async createPreset(preset?: Partial<PresetConfig>) {
      this.isLoading = true;
      this.error = null;
      try {
        const newPreset = {
          ...createEmptyPresetConfig(),
          ...preset,
        };
        const created = await invoke<PresetConfig>('create_preset', { preset: newPreset });
        this.presets.unshift(created);
        return created;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to create preset:', e);
        return null;
      } finally {
        this.isLoading = false;
      }
    },

    async updatePreset(preset: PresetConfig) {
      this.isLoading = true;
      this.error = null;
      try {
        const updated = await invoke<PresetConfig>('update_preset', { preset });
        const index = this.presets.findIndex(p => p.id === preset.id);
        if (index !== -1) {
          this.presets[index] = updated;
        }
        if (this.currentPreset?.id === preset.id) {
          this.currentPreset = updated;
        }
        return updated;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to update preset:', e);
        return null;
      } finally {
        this.isLoading = false;
      }
    },

    async deletePreset(id: string) {
      this.isLoading = true;
      this.error = null;
      try {
        await invoke('delete_preset', { id });
        this.presets = this.presets.filter(p => p.id !== id);
        if (this.currentPreset?.id === id) {
          this.currentPreset = null;
        }
      } catch (e) {
        this.error = String(e);
        console.error('Failed to delete preset:', e);
      } finally {
        this.isLoading = false;
      }
    },

    async toggleFavorite(id: string) {
      this.error = null;
      try {
        await invoke('toggle_favorite', { id });
        const preset = this.presets.find(p => p.id === id);
        if (preset) {
          preset.isFavorite = !preset.isFavorite;
        }
      } catch (e) {
        this.error = String(e);
        console.error('Failed to toggle favorite:', e);
      }
    },

    async incrementUseCount(id: string) {
      this.error = null;
      try {
        await invoke('increment_use_count', { id });
        const preset = this.presets.find(p => p.id === id);
        if (preset) {
          preset.useCount++;
        }
      } catch (e) {
        this.error = String(e);
        console.error('Failed to increment use count:', e);
      }
    },

    async searchPresets(query: string) {
      this.isLoading = true;
      this.error = null;
      try {
        const presets = await invoke<PresetConfig[]>('search_presets', { query });
        return presets;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to search presets:', e);
        return [];
      } finally {
        this.isLoading = false;
      }
    },

    async fetchTags() {
      this.error = null;
      try {
        const tags = await invoke<Tag[]>('get_all_tags');
        this.tags = tags;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to fetch tags:', e);
      }
    },

    async createTag(name: string, color: string = '#6366f1') {
      this.error = null;
      try {
        const tag = await invoke<Tag>('create_tag', { name, color });
        this.tags.push(tag);
        return tag;
      } catch (e) {
        this.error = String(e);
        console.error('Failed to create tag:', e);
        return null;
      }
    },

    async deleteTag(id: string) {
      this.error = null;
      try {
        await invoke('delete_tag', { id });
        this.tags = this.tags.filter(t => t.id !== id);
      } catch (e) {
        this.error = String(e);
        console.error('Failed to delete tag:', e);
      }
    },

    setFilter(filter: Partial<FilterOptions>) {
      this.filter = { ...this.filter, ...filter };
    },

    setCurrentPreset(preset: PresetConfig | null) {
      this.currentPreset = preset;
    },

    clearError() {
      this.error = null;
    },
  },
});
