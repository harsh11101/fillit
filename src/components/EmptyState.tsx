import { FileText, Plus } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { EmptyStateProps } from '@/types';


export function EmptyState({ onCreateNew }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center py-16 px-4 text-center">
      <div className="w-24 h-24 rounded-full bg-muted flex items-center justify-center mb-6">
        <FileText className="w-12 h-12 text-muted-foreground" />
      </div>
      <h3 className="text-2xl font-semibold mb-2">No snippets yet</h3>
      <p className="text-muted-foreground mb-6 max-w-md">
        Create your first text snippet to start automating your typing. Snippets expand instantly when you type their trigger text.
      </p>
      <Button onClick={onCreateNew} size="lg">
        <Plus className="w-4 h-4 mr-2" />
        Create Your First Snippet
      </Button>
    </div>
  );
}