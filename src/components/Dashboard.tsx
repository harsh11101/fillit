import { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core"
import { Sidebar } from './Sidebar';
import { Header } from './Header';
import { SnippetList } from './SnippetList';
import { SnippetEditor } from './SnippetEditor';
import { EmptyState } from './EmptyState';
import { StatsCards } from './StatsCard';
import { Snippet } from '@/types';
import { toast } from 'sonner';

export function Dashboard() {
  const [snippets, setSnippets] = useState<Snippet[]>([]);
  const [filteredSnippets, setFilteredSnippets] = useState<Snippet[]>([]);
  const [selectedSnippet, setSelectedSnippet] = useState<Snippet | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [_loading, setLoading] = useState(true);

  const loadSnippets = async () => {
    try {
      setLoading(true);
      const result = await invoke<Snippet[]>('get_all_snippets');
      setSnippets(result);
      setFilteredSnippets(result);
    } catch (error) {
      toast.error('Failed to load snippets');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadSnippets();
  }, []);

  useEffect(() => {
    let filtered = snippets;

    if (searchQuery) {
      filtered = filtered.filter(
        (s) =>
          s.trigger.toLowerCase().includes(searchQuery.toLowerCase()) ||
          s.content.toLowerCase().includes(searchQuery.toLowerCase()) ||
          s.description?.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    if (selectedTag) {
      filtered = filtered.filter((s) => s.tags.includes(selectedTag));
    }

    setFilteredSnippets(filtered);
  }, [searchQuery, selectedTag, snippets]);

  const handleCreateSnippet = async (
    trigger: string,
    content: string,
    description: string,
    tags: string[],
    isHtml: boolean
  ) => {
    try {
      await invoke('create_snippet', {
        trigger,
        content,
        description: description || null,
        tags,
        isHtml,
      });
      toast.success('Snippet created successfully');
      loadSnippets();
      setIsCreating(false);
    } catch (error) {
      toast.error(error as string);
    }
  };

  const handleUpdateSnippet = async (
    id: string,
    trigger: string,
    content: string,
    description: string,
    tags: string[],
    isHtml: boolean
  ) => {
    try {
      await invoke('update_snippet', {
        id,
        trigger,
        content,
        description: description || null,
        tags,
        isHtml,
      });
      toast.success('Snippet updated successfully');
      loadSnippets();
      setSelectedSnippet(null);
    } catch (error) {
      toast.error(error as string);
    }
  };

  const handleDeleteSnippet = async (id: string) => {
    try {
      await invoke('delete_snippet', { id });
      toast.success
      loadSnippets();
      setSelectedSnippet(null);
    } catch (error) {
      toast.error(error as string);
    }
  };

  const handleExport = async () => {
    try {
      const json = await invoke<string>('export_snippets');
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `snippets-${Date.now()}.json`;
      a.click();
      toast.success('Exported snippets successfully');
    } catch (error) {
      toast.error(error as string);
    }
  };

  const handleImport = async (file: File) => {
    try {
      const text = await file.text();
      const count = await invoke<number>('import_snippets', { json: text });
      toast.success(`Imported ${count} snippets successfully`);
      loadSnippets();
    } catch (error) {
      toast.error(error as string);
    }
  };

  const allTags = Array.from(new Set(snippets.flatMap((s) => s.tags)));

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      <Sidebar
        tags={allTags}
        selectedTag={selectedTag}
        onSelectTag={setSelectedTag}
        onCreateNew={() => {
          setIsCreating(true);
          setSelectedSnippet(null);
        }}
      />
      
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          onExport={handleExport}
          onImport={handleImport}
        />

        <div className="flex-1 overflow-auto p-6">
          {!isCreating && !selectedSnippet && (
            <>
              <StatsCards snippets={snippets} />
              {filteredSnippets.length === 0 ? (
                <EmptyState onCreateNew={() => setIsCreating(true)} />
              ) : (
                <SnippetList
                  snippets={filteredSnippets}
                  onSelectSnippet={(snippet) => {
                    setSelectedSnippet(snippet);
                    setIsCreating(false);
                  }}
                  onDeleteSnippet={handleDeleteSnippet}
                />
              )}
            </>
          )}

          {(isCreating || selectedSnippet) && (
            <SnippetEditor
              snippet={selectedSnippet}
              onSave={
                isCreating
                  ? handleCreateSnippet
                  : (trigger, content, description, tags) =>
                      handleUpdateSnippet(
                        selectedSnippet!.id,
                        trigger,
                        content,
                        description,
                        tags,
                        selectedSnippet!.is_html
                      )
              }
              onCancel={() => {
                setIsCreating(false);
                setSelectedSnippet(null);
              }}
            />
          )}
        </div>
      </div>
    </div>
  );
}