import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Badge } from '@/components/ui/badge';
import { X, Plus } from 'lucide-react';
import { SnippetEditorProps } from '@/types';


export function SnippetEditor({ snippet, onSave, onCancel }: SnippetEditorProps) {
  const [trigger, setTrigger] = useState('');
  const [content, setContent] = useState('');
  const [description, setDescription] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [newTag, setNewTag] = useState('');

  useEffect(() => {
    if (snippet) {
      setTrigger(snippet.trigger);
      setContent(snippet.content);
      setDescription(snippet.description || '');
      setTags(snippet.tags);
    }
  }, [snippet]);

  const handleAddTag = () => {
    if (newTag.trim() && !tags.includes(newTag.trim())) {
      setTags([...tags, newTag.trim()]);
      setNewTag('');
    }
  };

  const handleRemoveTag = (tagToRemove: string) => {
    setTags(tags.filter((t) => t !== tagToRemove));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (trigger.trim() && content.trim()) {
      onSave(trigger.trim(), content.trim(), description.trim(), tags);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <Card>
        <CardHeader>
          <CardTitle>{snippet ? 'Edit Snippet' : 'Create New Snippet'}</CardTitle>
          <CardDescription>
            {snippet
              ? 'Update your snippet details below'
              : 'Fill in the details to create a new text snippet'}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="trigger">Trigger Text *</Label>
              <Input
                id="trigger"
                placeholder="e.g., /email, ::hello"
                value={trigger}
                onChange={(e) => setTrigger(e.target.value)}
                className="font-mono"
                required
              />
              <p className="text-sm text-muted-foreground">
                The text you'll type to trigger this snippet
              </p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="content">Content *</Label>
              <Textarea
                id="content"
                placeholder="The text that will be inserted..."
                value={content}
                onChange={(e) => setContent(e.target.value)}
                className="font-mono min-h-[200px] resize-y"
                required
              />
              <p className="text-sm text-muted-foreground">
                {content.length} characters
              </p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="description">Description</Label>
              <Input
                id="description"
                placeholder="A brief description of this snippet"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label>Tags</Label>
              <div className="flex gap-2">
                <Input
                  placeholder="Add a tag..."
                  value={newTag}
                  onChange={(e) => setNewTag(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') {
                      e.preventDefault();
                      handleAddTag();
                    }
                  }}
                />
                <Button type="button" onClick={handleAddTag} variant="outline">
                  <Plus className="w-4 h-4" />
                </Button>
              </div>
              {tags.length > 0 && (
                <div className="flex flex-wrap gap-2 mt-3">
                  {tags.map((tag) => (
                    <Badge key={tag} variant="secondary" className="pl-3 pr-1">
                      {tag}
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon"
                        className="h-5 w-5 ml-1"
                        onClick={() => handleRemoveTag(tag)}
                      >
                        <X className="h-3 w-3" />
                      </Button>
                    </Badge>
                  ))}
                </div>
              )}
            </div>

            <div className="flex justify-end gap-3 pt-4">
              <Button type="button" variant="outline" onClick={onCancel}>
                Cancel
              </Button>
              <Button type="submit">
                {snippet ? 'Update Snippet' : 'Create Snippet'}
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}