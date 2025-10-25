import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Badge } from '@/components/ui/badge';
import { X, Plus, Eye, Code } from 'lucide-react';
import { SnippetEditorProps } from '@/types';
import { Switch } from '@/components/ui/switch';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { toast } from 'sonner';


export function SnippetEditor({ snippet, onSave, onCancel }: SnippetEditorProps) {
  const [trigger, setTrigger] = useState('');
  const [content, setContent] = useState('');
  const [description, setDescription] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [newTag, setNewTag] = useState('');
  const [isHtml, setIsHtml] = useState(false);
  const [previewTab, setPreviewTab] = useState<'code' | 'preview'>('code');

  useEffect(() => {
    if (snippet) {
      setTrigger(snippet.trigger);
      setContent(snippet.content);
      setDescription(snippet.description || '');
      setTags(snippet.tags);
      setIsHtml(snippet.is_html);
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
    
    // Validate trigger
    const trimmedTrigger = trigger.trim();
    
    if (!trimmedTrigger) {
      toast.error('Trigger cannot be empty');
      return;
    }
    
    if (trimmedTrigger.length > 100) {
      toast.error('Trigger cannot exceed 100 characters');
      return;
    }
    
    if (trimmedTrigger.includes(' ')) {
      toast.error('Trigger cannot contain spaces');
      return;
    }
    
    if (trimmedTrigger.includes('\n') || trimmedTrigger.includes('\r')) {
      toast.error('Trigger cannot contain newlines');
      return;
    }
    
    if (trimmedTrigger.includes('\t')) {
      toast.error('Trigger cannot contain tabs');
      return;
    }
    
    if (content.trim()) {
      onSave(trimmedTrigger, content.trim(), description.trim(), tags, isHtml);
    }
  };

  return (
    <div className="max-w-7xl mx-auto">
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Left Column - Form */}
          <Card>
            <CardHeader>
              <CardTitle>{snippet ? 'Edit Snippet' : 'Create New Snippet'}</CardTitle>
              <CardDescription>
                {snippet
                  ? 'Update your snippet details below'
                  : 'Fill in the details to create a new snippet'}
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
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
                <div className="flex items-center justify-between">
                  <Label htmlFor="is_html">HTML Content</Label>
                  <div className="flex items-center space-x-2">
                    <Switch
                      id="is_html"
                      checked={isHtml}
                      onCheckedChange={setIsHtml}
                    />
                    <Label htmlFor="is_html" className="text-sm cursor-pointer">
                      {isHtml ? 'Enabled' : 'Disabled'}
                    </Label>
                  </div>
                </div>
                <p className="text-sm text-muted-foreground">
                  Enable if your content contains HTML formatting
                </p>
              </div>

              <div className="space-y-2">
                <Label htmlFor="content">Content *</Label>
                <Textarea
                  id="content"
                  placeholder={
                    isHtml
                      ? '<p>Your HTML content here...</p>'
                      : 'The text that will be inserted...'
                  }
                  value={content}
                  onChange={(e) => setContent(e.target.value)}
                  className="font-mono min-h-[300px] resize-y"
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
            </CardContent>
          </Card>

          {/* Right Column - Preview */}
          <Card>
            <CardHeader>
              <CardTitle>Preview</CardTitle>
              <CardDescription>
                See how your content will appear when inserted
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Tabs value={previewTab} onValueChange={(v) => setPreviewTab(v as 'code' | 'preview')}>
                <TabsList className="grid w-full grid-cols-2">
                  <TabsTrigger value="code" className="flex items-center gap-2">
                    <Code className="w-4 h-4" />
                    Code
                  </TabsTrigger>
                  <TabsTrigger value="preview" className="flex items-center gap-2" disabled={!isHtml}>
                    <Eye className="w-4 h-4" />
                    Preview
                  </TabsTrigger>
                </TabsList>

                <TabsContent value="code" className="mt-4">
                  <div className="border rounded-lg p-4 bg-muted/50 font-mono text-sm min-h-[400px] max-h-[600px] overflow-auto whitespace-pre-wrap break-words">
                    {content || <span className="text-muted-foreground">Your content will appear here...</span>}
                  </div>
                </TabsContent>

                <TabsContent value="preview" className="mt-4">
                  {isHtml ? (
                    <div className="border rounded-lg p-4 bg-background min-h-[400px] max-h-[600px] overflow-auto">
                      {content ? (
                        <div
                          className="prose prose-sm dark:prose-invert max-w-none"
                          dangerouslySetInnerHTML={{ __html: content }}
                        />
                      ) : (
                        <p className="text-muted-foreground">Your HTML preview will appear here...</p>
                      )}
                    </div>
                  ) : (
                    <div className="border rounded-lg p-4 bg-muted/50 min-h-[400px] flex items-center justify-center">
                      <p className="text-muted-foreground">Enable HTML content to see preview</p>
                    </div>
                  )}
                </TabsContent>
              </Tabs>

              {isHtml && content && (
                <div className="mt-4 p-3 bg-amber-50 dark:bg-amber-950 border border-amber-200 dark:border-amber-800 rounded-lg">
                  <p className="text-sm text-amber-900 dark:text-amber-200">
                    <strong>Note:</strong> HTML content will be rendered as-is. Make sure your HTML is valid to avoid rendering issues.
                  </p>
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      </form>
    </div>
  );
}