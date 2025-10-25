import { Plus, Tag, Home, X, Settings as SettingsIcon } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { SidebarProps } from '@/types';

export function Sidebar({tags, selectedTag, onSelectTag, onCreateNew, currentView, onViewChange, onBackToSnippets}: SidebarProps) {
  return (
    <div className="w-64 border-r bg-card flex flex-col">
      <div className="p-6 border-b">
        <h1 className="text-2xl font-bold bg-gradient-to-r from-primary to-primary/60 bg-clip-text text-transparent">
          Lemme Do It
        </h1>
        <p className="text-sm text-muted-foreground mt-1">Text Snippet Manager</p>
      </div>

      <div className="p-4">
        <Button onClick={onCreateNew} className="w-full" size="lg" disabled={currentView === 'settings'}>
          <Plus className="w-4 h-4 mr-2" />
          New Snippet
        </Button>
      </div>

      <ScrollArea className="flex-1 px-4">
        <div className="space-y-2">
          <Button
            variant={currentView === 'snippets' && selectedTag === null ? 'secondary' : 'ghost'}
            className="w-full justify-start"
            onClick={() => {
              onViewChange('snippets');
              onSelectTag(null);
              onBackToSnippets(); // Close any open editor
            }}
          >
            <Home className="w-4 h-4 mr-2" />
            All Snippets
          </Button>

          <Button
            variant={currentView === 'settings' ? 'secondary' : 'ghost'}
            className="w-full justify-start"
            onClick={() => onViewChange('settings')}
          >
            <SettingsIcon className="w-4 h-4 mr-2" />
            Settings
          </Button>

          {currentView === 'snippets' && tags.length > 0 && (
            <>
              <div className="pt-4 pb-2">
                <h3 className="text-sm font-semibold text-muted-foreground px-2">Tags</h3>
              </div>
              {tags.map((tag) => (
                <Button
                  key={tag}
                  variant={selectedTag === tag ? 'secondary' : 'ghost'}
                  className="w-full justify-start group"
                  onClick={() => {
                    onSelectTag(selectedTag === tag ? null : tag);
                    onBackToSnippets(); // Close any open editor when switching tags
                  }}
                >
                  <Tag className="w-4 h-4 mr-2" />
                  <span className="flex-1 text-left truncate">{tag}</span>
                  {selectedTag === tag && (
                    <X className="w-3 h-3 opacity-0 group-hover:opacity-100 transition-opacity" />
                  )}
                </Button>
              ))}
            </>
          )}
        </div>
      </ScrollArea>

      <div className="p-4 border-t text-xs text-muted-foreground text-center">
        <p>Press triggers to expand</p>
      </div>
    </div>
  );
}