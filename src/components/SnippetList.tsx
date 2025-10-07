import { SnippetListProps } from '@/types';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Trash2, Edit, Copy } from 'lucide-react';
import { toast } from 'sonner';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';


export function SnippetList({ snippets, onSelectSnippet, onDeleteSnippet }: SnippetListProps) {

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    toast.success('Snippet copied to clipboard');
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  };

  return (
    <div className="mt-6">
      <h2 className="text-2xl font-semibold mb-4">Your Snippets</h2>
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {snippets.map((snippet) => (
          <Card key={snippet.id} className="group hover:shadow-lg transition-all duration-200">
            <CardHeader>
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <CardTitle className="text-lg font-mono">{snippet.trigger}</CardTitle>
                  {snippet.description && (
                    <CardDescription className="mt-1">{snippet.description}</CardDescription>
                  )}
                </div>
                <div className="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-8 w-8"
                    onClick={() => onSelectSnippet(snippet)}
                  >
                    <Edit className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-8 w-8"
                    onClick={() => copyToClipboard(snippet.content)}
                  >
                    <Copy className="h-4 w-4" />
                  </Button>
                  <AlertDialog>
                    <AlertDialogTrigger asChild>
                      <Button variant="ghost" size="icon" className="h-8 w-8 text-destructive">
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent>
                      <AlertDialogHeader>
                        <AlertDialogTitle>Delete Snippet?</AlertDialogTitle>
                        <AlertDialogDescription>
                          This will permanently delete the snippet "{snippet.trigger}". This action cannot be undone.
                        </AlertDialogDescription>
                      </AlertDialogHeader>
                      <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction onClick={() => onDeleteSnippet(snippet.id)}>
                          Delete
                        </AlertDialogAction>
                      </AlertDialogFooter>
                    </AlertDialogContent>
                  </AlertDialog>
                </div>
              </div>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                <div className="bg-muted p-3 rounded-md font-mono text-sm max-h-24 overflow-auto">
                  {snippet.content.length > 150
                    ? snippet.content.substring(0, 150) + '...'
                    : snippet.content}
                </div>
                
                <div className="flex items-center justify-between text-xs text-muted-foreground">
                  <span>Used {snippet.usage_count} times</span>
                  <span>{formatDate(snippet.updated_at)}</span>
                </div>

                {snippet.tags.length > 0 && (
                  <div className="flex flex-wrap gap-1">
                    {snippet.tags.map((tag) => (
                      <Badge key={tag} variant="secondary" className="text-xs">
                        {tag}
                      </Badge>
                    ))}
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}