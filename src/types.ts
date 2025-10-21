export type Snippet = {
    id: string;
    trigger: string;
    content: string;
    description: string | null;
    tags: string[];
    usage_count: number;
    created_at: number;
    updated_at: number;
    is_html: boolean;
}

export type Theme = 'dark' | 'light';

export type ThemeProviderProps = {
  children: React.ReactNode;
  defaultTheme?: Theme;
  storageKey?: string;
};

export type ThemeProviderState = {
  theme: Theme;
  setTheme: (theme: Theme) => void;
};

export interface SnippetListProps {
  snippets: Snippet[];
  onSelectSnippet: (snippet: Snippet) => void;
  onDeleteSnippet: (id: string) => void;
}

export interface StatsCardsProps {
  snippets: Snippet[];
}

export interface EmptyStateProps {
  onCreateNew: () => void;
}

export interface HeaderProps {
  searchQuery: string;
  onSearchChange: (query: string) => void;
  onExport: () => void;
  onImport: (file: File) => void;
}

export interface SidebarProps {
  tags: string[];
  selectedTag: string | null;
  onSelectTag: (tag: string | null) => void;
  onCreateNew: () => void;
}

export interface SnippetEditorProps {
  snippet: Snippet | null;
  onSave: (trigger: string, content: string, description: string, tags: string[], isHtml: boolean) => void;
  onCancel: () => void;
}